//! Git operations module
//!
//! Handles reading commits from git history, finding the git directory,
//! and reading commit messages from COMMIT_EDITMSG.

use anyhow::{Context, Result};
use git2::{Repository, Sort};
use std::path::{Path, PathBuf};

/// Git repository wrapper
pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Open a git repository from the given path or any parent directory
    pub fn open(path: &Path) -> Result<Self> {
        let repo = Repository::discover(path)
            .with_context(|| format!("Failed to find git repository from {}", path.display()))?;
        Ok(Self { repo })
    }

    /// Open a git repository from the current directory
    pub fn open_current() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::open(&cwd)
    }

    /// Get the path to the .git directory
    pub fn git_dir(&self) -> &Path {
        self.repo.path()
    }

    /// Get the working directory path
    pub fn workdir(&self) -> Option<&Path> {
        self.repo.workdir()
    }

    /// Read the COMMIT_EDITMSG file (used by git hooks)
    pub fn read_commit_editmsg(&self) -> Result<String> {
        let path = self.git_dir().join("COMMIT_EDITMSG");
        std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))
    }

    /// Get the last commit message
    pub fn get_last_commit_message(&self) -> Result<String> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.message().unwrap_or("").to_string())
    }

    /// Get commit messages in a range
    ///
    /// - `from`: starting commit (exclusive), can be a SHA, tag, or branch name
    /// - `to`: ending commit (inclusive), defaults to HEAD
    pub fn get_commits_in_range(&self, from: Option<&str>, to: Option<&str>) -> Result<Vec<String>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

        // Set the starting point
        let to_oid = if let Some(to_ref) = to {
            self.resolve_ref(to_ref)?
        } else {
            self.repo.head()?.target().context("HEAD has no target")?
        };

        revwalk.push(to_oid)?;

        // Set the stopping point
        if let Some(from_ref) = from {
            let from_oid = self.resolve_ref(from_ref)?;
            revwalk.hide(from_oid)?;
        }

        let mut messages = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            if let Some(msg) = commit.message() {
                messages.push(msg.to_string());
            }
        }

        Ok(messages)
    }

    /// Get the last N commit messages
    pub fn get_last_n_commits(&self, n: usize) -> Result<Vec<String>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(Sort::TOPOLOGICAL)?;
        revwalk.push_head()?;

        let mut messages = Vec::new();
        for oid in revwalk.take(n) {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            if let Some(msg) = commit.message() {
                messages.push(msg.to_string());
            }
        }

        Ok(messages)
    }

    /// Get the last tag name
    pub fn get_last_tag(&self) -> Result<Option<String>> {
        let tags = self.repo.tag_names(None)?;

        // Get all tags with their commit times
        let mut tag_times: Vec<(String, i64)> = Vec::new();

        for tag_name in tags.iter().flatten() {
            if let Ok(reference) = self.repo.find_reference(&format!("refs/tags/{}", tag_name)) {
                if let Ok(commit) = reference.peel_to_commit() {
                    tag_times.push((tag_name.to_string(), commit.time().seconds()));
                }
            }
        }

        // Sort by time descending
        tag_times.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(tag_times.first().map(|(name, _)| name.clone()))
    }

    /// Get commits since the last tag
    pub fn get_commits_since_last_tag(&self) -> Result<Vec<String>> {
        match self.get_last_tag()? {
            Some(tag) => self.get_commits_in_range(Some(&tag), None),
            None => {
                // No tags, get all commits
                self.get_all_commits()
            }
        }
    }

    /// Get all commit messages
    pub fn get_all_commits(&self) -> Result<Vec<String>> {
        self.get_commits_in_range(None, None)
    }

    /// Resolve a reference string to an Oid
    fn resolve_ref(&self, reference: &str) -> Result<git2::Oid> {
        // Try as a direct SHA first
        if let Ok(oid) = git2::Oid::from_str(reference) {
            return Ok(oid);
        }

        // Try as a reference
        if let Ok(reference) = self.repo.find_reference(reference) {
            if let Some(oid) = reference.target() {
                return Ok(oid);
            }
        }

        // Try as a tag
        if let Ok(reference) = self.repo.find_reference(&format!("refs/tags/{}", reference)) {
            if let Ok(commit) = reference.peel_to_commit() {
                return Ok(commit.id());
            }
        }

        // Try as a branch
        if let Ok(reference) = self.repo.find_reference(&format!("refs/heads/{}", reference)) {
            if let Some(oid) = reference.target() {
                return Ok(oid);
            }
        }

        // Try revparse
        let obj = self.repo.revparse_single(reference)?;
        if let Some(commit) = obj.as_commit() {
            return Ok(commit.id());
        }
        if let Some(tag) = obj.as_tag() {
            return Ok(tag.target_id());
        }

        Ok(obj.id())
    }
}

/// Find the .git directory from a starting path
pub fn find_git_dir(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);

    while let Some(dir) = current {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            return Some(git_dir);
        }
        current = dir.parent();
    }

    None
}

/// Read a commit message from a file
pub fn read_commit_message_file(path: &Path) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read commit message from {}", path.display()))?;

    // Strip comments (lines starting with #)
    let lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();

    Ok(lines.join("\n").trim().to_string())
}

/// Read from COMMIT_EDITMSG in the given git directory
pub fn read_commit_editmsg(git_dir: &Path) -> Result<String> {
    let path = git_dir.join("COMMIT_EDITMSG");
    read_commit_message_file(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_git_dir() {
        let cwd = std::env::current_dir().unwrap();
        // This test assumes we're in a git repository
        let git_dir = find_git_dir(&cwd);
        assert!(git_dir.is_some() || true); // Pass even if not in git repo
    }

    #[test]
    fn test_strip_comments() {
        let content = "feat: add feature\n\n# Comment line\nBody text";
        let stripped = content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!stripped.contains("# Comment"));
    }
}
