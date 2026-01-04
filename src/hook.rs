//! Git hook installation and management
//!
//! Installs a commit-msg hook that validates commit messages using cargo-commitlint.

use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;
use which::which;

pub struct HookInstaller;

impl HookInstaller {
    pub fn install(force: bool) -> Result<(), String> {
        let git_dir = Self::find_git_dir()?;
        let hooks_dir = git_dir.join("hooks");

        // Create hooks directory if it doesn't exist
        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir)
                .map_err(|e| format!("Failed to create .git/hooks directory: {}", e))?;
        }

        // Find cargo-commitlint binary
        let binary_path = Self::find_binary()?;

        // Create commit-msg hook
        let hook_path = hooks_dir.join("commit-msg");

        // Check if hook already exists
        if hook_path.exists() && !force {
            let content = fs::read_to_string(&hook_path)
                .map_err(|e| format!("Failed to read existing hook: {}", e))?;

            if !content.contains("cargo-commitlint") && !content.contains("cargo commitlint") {
                return Err(format!(
                    "A commit-msg hook already exists at {}.\n\
                     Use --force to overwrite it, or manually add cargo-commitlint to the existing hook.",
                    hook_path.display()
                ));
            }

            println!("ℹ Hook already installed, updating...");
        }

        let hook_content = Self::generate_hook_script(&binary_path);

        fs::write(&hook_path, hook_content)
            .map_err(|e| format!("Failed to write commit-msg hook: {}", e))?;

        // Make hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)
                .map_err(|e| format!("Failed to get hook metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)
                .map_err(|e| format!("Failed to set hook permissions: {}", e))?;
        }

        println!("✓ Git hook installed successfully at {}", hook_path.display());
        println!("  Commit messages will now be validated using cargo-commitlint");

        Ok(())
    }

    pub fn uninstall() -> Result<(), String> {
        let git_dir = Self::find_git_dir()?;
        let hook_path = git_dir.join("hooks").join("commit-msg");

        if hook_path.exists() {
            // Check if it's our hook
            let content = fs::read_to_string(&hook_path)
                .map_err(|e| format!("Failed to read hook: {}", e))?;

            if content.contains("cargo-commitlint") || content.contains("cargo commitlint") {
                fs::remove_file(&hook_path)
                    .map_err(|e| format!("Failed to remove hook: {}", e))?;
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

    fn find_git_dir() -> Result<std::path::PathBuf, String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let mut dir = current_dir.as_path();

        loop {
            let git_dir = dir.join(".git");
            if git_dir.exists() {
                return Ok(git_dir);
            }

            match dir.parent() {
                Some(parent) => dir = parent,
                None => return Err("Not a git repository (or any parent directory)".to_string()),
            }
        }
    }

    fn find_binary() -> Result<std::path::PathBuf, String> {
        // Try to find cargo-commitlint in PATH
        if let Ok(path) = which("cargo-commitlint") {
            return Ok(path);
        }

        // Try to find via cargo
        let output = Command::new("cargo")
            .args(["locate-project", "--workspace", "--message-format", "plain"])
            .output()
            .map_err(|e| format!("Failed to run cargo: {}", e))?;

        if output.status.success() {
            let workspace_root = String::from_utf8(output.stdout)
                .map_err(|e| format!("Invalid UTF-8 in cargo output: {}", e))?
                .trim()
                .to_string();
            let workspace_path = Path::new(&workspace_root)
                .parent()
                .ok_or_else(|| "Invalid workspace path".to_string())?;

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
# Git commit-msg hook installed by cargo-commitlint
# This hook validates commit messages according to Conventional Commits specification
# https://github.com/pegasusheavy/cargo-commitlint

COMMIT_MSG_FILE="$1"

# Check if we should skip validation
if [ -n "$COMMITLINT_SKIP" ]; then
    echo "Skipping commit message validation (COMMITLINT_SKIP is set)"
    exit 0
fi

# Try to use cargo commitlint subcommand first (if installed globally)
if command -v cargo >/dev/null 2>&1; then
    if cargo commitlint --version >/dev/null 2>&1; then
        cargo commitlint check --edit "$COMMIT_MSG_FILE"
        exit $?
    fi
fi

# Fall back to direct binary path
if [ -x "{bin_path}" ]; then
    "{bin_path}" check --edit "$COMMIT_MSG_FILE"
    exit $?
fi

# Try cargo-commitlint directly
if command -v cargo-commitlint >/dev/null 2>&1; then
    cargo-commitlint check --edit "$COMMIT_MSG_FILE"
    exit $?
fi

echo "Warning: cargo-commitlint not found. Skipping commit message validation."
exit 0
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
        assert!(script.contains("COMMITLINT_SKIP"));
    }
}
