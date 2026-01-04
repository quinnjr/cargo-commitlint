//! Build script for cargo-commitlint
//!
//! This script automatically installs git hooks when the crate is built,
//! similar to how cargo-husky works. It enables zero-config hook installation
//! by simply adding cargo-commitlint as a dev-dependency.
//!
//! Configuration can be done via Cargo.toml:
//! ```toml
//! [package.metadata.commitlint]
//! # Disable auto-installation of hooks
//! no-install = false
//! # Install in user-hooks mode (creates hooks in .commitlint/hooks/)
//! user-hooks = true
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Only run hook installation for dev builds or when explicitly enabled
    // Skip for release builds unless forced
    let _profile = env::var("PROFILE").unwrap_or_default();

    // Check if we should skip installation
    if should_skip_installation() {
        return;
    }

    // Find the git directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let git_dir = match find_git_dir(Path::new(&manifest_dir)) {
        Some(dir) => dir,
        None => {
            // Not a git repository, skip silently
            return;
        }
    };

    // Check configuration from Cargo.toml metadata
    let config = read_config();

    if config.no_install {
        return;
    }

    // Determine hooks directory
    let hooks_dir = if config.user_hooks {
        // User hooks mode: create .commitlint/hooks/ directory
        let project_root = git_dir.parent().unwrap_or(Path::new("."));
        let user_hooks_dir = project_root.join(".commitlint").join("hooks");

        // Also need to configure git to use this hooks directory
        configure_git_hooks_path(&git_dir, &user_hooks_dir);

        user_hooks_dir
    } else {
        // Standard mode: use .git/hooks/
        git_dir.join("hooks")
    };

    // Install the commit-msg hook
    if let Err(e) = install_commit_msg_hook(&hooks_dir, &config) {
        // Print warning but don't fail the build
        println!("cargo:warning=Failed to install commit-msg hook: {}", e);
    }

    // Ensure rebuild if hook files change
    println!("cargo:rerun-if-changed=.commitlint/hooks/commit-msg");
    println!("cargo:rerun-if-changed=.git/hooks/commit-msg");
    println!("cargo:rerun-if-env-changed=COMMITLINT_NO_INSTALL");
}

/// Configuration read from Cargo.toml metadata
#[derive(Default)]
struct Config {
    no_install: bool,
    user_hooks: bool,
    run_cargo_fmt: bool,
    run_cargo_clippy: bool,
    run_cargo_test: bool,
}

fn should_skip_installation() -> bool {
    // Skip if COMMITLINT_NO_INSTALL env var is set
    if env::var("COMMITLINT_NO_INSTALL").is_ok() {
        return true;
    }

    // Skip in CI environments unless explicitly enabled
    if env::var("CI").is_ok() && env::var("COMMITLINT_INSTALL_IN_CI").is_err() {
        return true;
    }

    false
}

fn read_config() -> Config {
    let mut config = Config::default();

    // Try to read from environment variables first
    if env::var("COMMITLINT_NO_INSTALL").is_ok() {
        config.no_install = true;
    }
    if env::var("COMMITLINT_USER_HOOKS").is_ok() {
        config.user_hooks = true;
    }

    // Try to read Cargo.toml metadata
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");

    if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
        // Simple parsing - look for [package.metadata.commitlint] section
        let mut in_section = false;
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[package.metadata.commitlint]" {
                in_section = true;
                continue;
            }

            if in_section {
                if trimmed.starts_with('[') {
                    break; // End of section
                }

                if trimmed.starts_with("no-install") && trimmed.contains("true") {
                    config.no_install = true;
                }
                if trimmed.starts_with("user-hooks") && trimmed.contains("true") {
                    config.user_hooks = true;
                }
                if trimmed.starts_with("run-cargo-fmt") && trimmed.contains("true") {
                    config.run_cargo_fmt = true;
                }
                if trimmed.starts_with("run-cargo-clippy") && trimmed.contains("true") {
                    config.run_cargo_clippy = true;
                }
                if trimmed.starts_with("run-cargo-test") && trimmed.contains("true") {
                    config.run_cargo_test = true;
                }
            }
        }
    }

    config
}

fn find_git_dir(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);

    while let Some(dir) = current {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            // Handle both regular .git directory and gitdir file (worktrees)
            if git_dir.is_file() {
                // It's a gitdir file, read the actual path
                if let Ok(content) = fs::read_to_string(&git_dir) {
                    if let Some(path) = content.strip_prefix("gitdir: ") {
                        return Some(PathBuf::from(path.trim()));
                    }
                }
            }
            return Some(git_dir);
        }
        current = dir.parent();
    }

    None
}

fn configure_git_hooks_path(git_dir: &Path, hooks_dir: &Path) {
    // Set core.hooksPath to use the custom hooks directory
    let project_root = git_dir.parent().unwrap_or(Path::new("."));

    // Make the path relative to project root
    let relative_path = hooks_dir
        .strip_prefix(project_root)
        .unwrap_or(hooks_dir)
        .to_string_lossy();

    let _ = Command::new("git")
        .args(["config", "core.hooksPath", &relative_path])
        .current_dir(project_root)
        .output();
}

fn install_commit_msg_hook(hooks_dir: &Path, config: &Config) -> Result<(), String> {
    // Create hooks directory if it doesn't exist
    fs::create_dir_all(hooks_dir)
        .map_err(|e| format!("Failed to create hooks directory: {}", e))?;

    let hook_path = hooks_dir.join("commit-msg");

    // Check if hook already exists
    if hook_path.exists() {
        let content = fs::read_to_string(&hook_path).unwrap_or_default();

        // If it's already our hook, check if it needs updating
        if content.contains("cargo-commitlint") || content.contains("cargo commitlint") {
            // Already installed, skip
            return Ok(());
        }

        // There's an existing hook that's not ours
        // Append our validation to it instead of overwriting
        let updated_content = append_to_existing_hook(&content);
        fs::write(&hook_path, updated_content)
            .map_err(|e| format!("Failed to update hook: {}", e))?;
    } else {
        // Create new hook
        let hook_content = generate_hook_script(config);
        fs::write(&hook_path, hook_content)
            .map_err(|e| format!("Failed to write hook: {}", e))?;
    }

    // Make hook executable on Unix
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

    println!("cargo:warning=cargo-commitlint: Installed commit-msg hook at {}", hook_path.display());

    Ok(())
}

fn generate_hook_script(config: &Config) -> String {
    let mut script = String::from(r#"#!/bin/sh
# Git commit-msg hook installed by cargo-commitlint
# This hook validates commit messages according to Conventional Commits specification
# https://github.com/pegasusheavy/cargo-commitlint
#
# To disable this hook, set COMMITLINT_SKIP=1 environment variable

COMMIT_MSG_FILE="$1"

# Skip if COMMITLINT_SKIP is set
if [ -n "$COMMITLINT_SKIP" ]; then
    exit 0
fi

"#);

    // Add optional pre-commit checks
    if config.run_cargo_fmt {
        script.push_str(r#"
# Run cargo fmt check
if command -v cargo >/dev/null 2>&1; then
    cargo fmt --check || {
        echo "cargo fmt check failed. Run 'cargo fmt' to fix formatting."
        exit 1
    }
fi
"#);
    }

    if config.run_cargo_clippy {
        script.push_str(r#"
# Run cargo clippy
if command -v cargo >/dev/null 2>&1; then
    cargo clippy --all-targets --all-features -- -D warnings || {
        echo "cargo clippy found warnings/errors."
        exit 1
    }
fi
"#);
    }

    script.push_str(r#"
# Validate commit message with cargo-commitlint
if command -v cargo >/dev/null 2>&1; then
    # Try cargo subcommand first
    if cargo commitlint --version >/dev/null 2>&1; then
        cargo commitlint check --edit "$COMMIT_MSG_FILE"
        exit $?
    fi
fi

# Try direct binary
if command -v cargo-commitlint >/dev/null 2>&1; then
    cargo-commitlint check --edit "$COMMIT_MSG_FILE"
    exit $?
fi

# cargo-commitlint not found, skip validation with warning
echo "Warning: cargo-commitlint not found in PATH. Skipping commit message validation."
echo "Install with: cargo install cargo-commitlint"
exit 0
"#);

    script
}

fn append_to_existing_hook(existing: &str) -> String {
    let mut result = existing.to_string();

    // Don't add if already present
    if result.contains("cargo-commitlint") || result.contains("cargo commitlint") {
        return result;
    }

    result.push_str(r#"

# ============================================================
# cargo-commitlint validation (appended automatically)
# ============================================================
if [ -z "$COMMITLINT_SKIP" ]; then
    if command -v cargo >/dev/null 2>&1 && cargo commitlint --version >/dev/null 2>&1; then
        cargo commitlint check --edit "$1" || exit $?
    elif command -v cargo-commitlint >/dev/null 2>&1; then
        cargo-commitlint check --edit "$1" || exit $?
    fi
fi
"#);

    result
}
