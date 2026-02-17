use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use which::which;

pub struct HookInstaller;

impl HookInstaller {
    pub fn install() -> Result<()> {
        let git_dir = Self::find_git_dir()?;
        let hooks_dir = git_dir.join("hooks");

        // Create hooks directory if it doesn't exist
        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir).context("Failed to create .git/hooks directory")?;
        }

        // Find cargo-commitlint binary
        let binary_path = Self::find_binary()?;

        // Create commit-msg hook
        let hook_path = hooks_dir.join("commit-msg");
        let hook_content = Self::generate_hook_script(&binary_path);

        fs::write(&hook_path, hook_content).context("Failed to write commit-msg hook")?;

        // Make hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }

        println!(
            "✓ Git hook installed successfully at {}",
            hook_path.display()
        );
        println!("  Commit messages will now be validated using cargo-commitlint");

        Ok(())
    }

    pub fn uninstall() -> Result<()> {
        let git_dir = Self::find_git_dir()?;
        let hook_path = git_dir.join("hooks").join("commit-msg");

        if hook_path.exists() {
            // Check if it's our hook
            let content = fs::read_to_string(&hook_path)?;
            if content.contains("cargo-commitlint") {
                fs::remove_file(&hook_path)?;
                println!("✓ Git hook uninstalled successfully");
            } else {
                println!("⚠ Hook exists but doesn't appear to be from cargo-commitlint");
                println!("  Skipping removal to avoid breaking other hooks");
            }
        } else {
            println!("ℹ No commit-msg hook found");
        }

        Ok(())
    }

    fn find_git_dir() -> Result<std::path::PathBuf> {
        let current_dir = std::env::current_dir()?;
        let mut dir = current_dir.as_path();

        loop {
            let git_dir = dir.join(".git");
            if git_dir.exists() {
                return Ok(git_dir);
            }

            match dir.parent() {
                Some(parent) => dir = parent,
                None => anyhow::bail!("Not a git repository (or any parent directory)"),
            }
        }
    }

    fn find_binary() -> Result<std::path::PathBuf> {
        // Try to find cargo-commitlint in PATH
        if let Ok(path) = which("cargo-commitlint") {
            return Ok(path);
        }

        // Try to find via cargo
        let output = Command::new("cargo")
            .args(&["locate-project", "--workspace", "--message-format", "plain"])
            .output()?;

        if output.status.success() {
            let workspace_root = String::from_utf8(output.stdout)?.trim().to_string();
            let workspace_path = Path::new(&workspace_root)
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Invalid workspace path"))?;

            // Check target/release or target/debug
            let release_path = workspace_path
                .join("target")
                .join("release")
                .join("cargo-commitlint");
            if release_path.exists() {
                return Ok(release_path);
            }

            let debug_path = workspace_path
                .join("target")
                .join("debug")
                .join("cargo-commitlint");
            if debug_path.exists() {
                return Ok(debug_path);
            }
        }

        // Fallback: assume it's in PATH with a different name
        // This handles the case where it's installed via cargo install
        Ok(std::path::PathBuf::from("cargo-commitlint"))
    }

    fn generate_hook_script(binary_path: &Path) -> String {
        let path_str = binary_path.to_string_lossy();

        format!(
            r#"#!/bin/sh
# Git commit-msg hook installed by cargo commitlint
# This hook validates commit messages according to Conventional Commits specification

COMMIT_MSG_FILE="$1"

# Try to use cargo commitlint subcommand first (if installed)
if command -v cargo >/dev/null 2>&1 && cargo commitlint --version >/dev/null 2>&1; then
    # Use cargo commitlint subcommand
    cat "$COMMIT_MSG_FILE" | cargo commitlint check
    exit $?
else
    # Fall back to direct binary path
    cat "$COMMIT_MSG_FILE" | {bin_path} check
    exit $?
fi
"#,
            bin_path = path_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_script_generation() {
        let path = Path::new("/usr/local/bin/cargo-commitlint");
        let script = HookInstaller::generate_hook_script(path);
        assert!(script.contains("cargo-commitlint"));
        assert!(script.contains("/usr/local/bin/cargo-commitlint"));
    }
}
