use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

/// Highest numeric suffix considered when looking for a free backup file name.
const MAX_BACKUP_SUFFIX: u32 = 100;

pub fn install() -> Result<()> {
    let git_dir = find_git_dir()?;
    let hooks_dir = git_dir.join("hooks");

    // Find cargo-commitlint binary
    let binary_path = find_binary()?;

    install_to(&hooks_dir, &binary_path)
}

pub fn uninstall() -> Result<()> {
    let git_dir = find_git_dir()?;
    uninstall_from(&git_dir.join("hooks"))
}

/// Write the commit-msg hook into `hooks_dir`, backing up any foreign hook first.
fn install_to(hooks_dir: &Path, binary_path: &Path) -> Result<()> {
    // Create hooks directory if it doesn't exist
    if !hooks_dir.exists() {
        fs::create_dir_all(hooks_dir).context("Failed to create .git/hooks directory")?;
    }

    // Create commit-msg hook
    let hook_path = hooks_dir.join("commit-msg");
    let hook_content = generate_hook_script(binary_path);

    // Back up an existing hook that isn't ours before overwriting it
    if hook_path.exists() {
        let existing =
            fs::read_to_string(&hook_path).context("Failed to read existing commit-msg hook")?;
        if !existing.contains("cargo-commitlint") {
            let backup_path = free_backup_path(hooks_dir)?;
            fs::rename(&hook_path, &backup_path)
                .context("Failed to back up existing commit-msg hook")?;
            println!(
                "ℹ Existing commit-msg hook backed up to {}",
                backup_path.display()
            );
        }
    }

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

/// Remove our commit-msg hook from `hooks_dir` and restore a previous backup, if any.
fn uninstall_from(hooks_dir: &Path) -> Result<()> {
    let hook_path = hooks_dir.join("commit-msg");

    if hook_path.exists() {
        // Check if it's our hook
        let content =
            fs::read_to_string(&hook_path).context("Failed to read existing commit-msg hook")?;
        if content.contains("cargo-commitlint") {
            fs::remove_file(&hook_path).context("Failed to remove commit-msg hook")?;
            println!("✓ Git hook uninstalled successfully");
            restore_backup(hooks_dir, &hook_path)?;
        } else {
            println!("⚠ Hook exists but doesn't appear to be from cargo-commitlint");
            println!("  Skipping removal to avoid breaking other hooks");
        }
    } else {
        println!("ℹ No commit-msg hook found");
    }

    Ok(())
}

/// Pick a backup path that does not already exist, so an earlier backup is never destroyed.
fn free_backup_path(hooks_dir: &Path) -> Result<PathBuf> {
    let base = hooks_dir.join("commit-msg.backup");
    if !base.exists() {
        return Ok(base);
    }

    for suffix in 1..=MAX_BACKUP_SUFFIX {
        let candidate = hooks_dir.join(format!("commit-msg.backup.{suffix}"));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    anyhow::bail!(
        "Refusing to overwrite an existing hook backup: {} and .1 through .{} all exist. \
         Remove or archive the old backups, then run the install again.",
        base.display(),
        MAX_BACKUP_SUFFIX
    )
}

/// Restore the un-suffixed `commit-msg.backup` (if present) and report any numbered backups.
fn restore_backup(hooks_dir: &Path, hook_path: &Path) -> Result<()> {
    let backup_path = hooks_dir.join("commit-msg.backup");
    if backup_path.exists() {
        fs::rename(&backup_path, hook_path)
            .context("Failed to restore backed-up commit-msg hook")?;
        println!(
            "✓ Previous commit-msg hook restored from {}",
            backup_path.display()
        );
    }

    let numbered = numbered_backups(hooks_dir);
    if !numbered.is_empty() {
        println!("ℹ Additional commit-msg hook backups were left in place:");
        for path in numbered {
            println!("    {}", path.display());
        }
    }

    Ok(())
}

/// Collect `commit-msg.backup.N` files, sorted by name. Never restored automatically.
fn numbered_backups(hooks_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(hooks_dir) else {
        return Vec::new();
    };

    let mut backups: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("commit-msg.backup."))
        })
        .collect();
    backups.sort();
    backups
}

fn find_git_dir() -> Result<PathBuf> {
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

fn find_binary() -> Result<PathBuf> {
    // Try to find cargo-commitlint in PATH
    if let Ok(path) = which("cargo-commitlint") {
        return Ok(path);
    }

    // Try to find via cargo
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
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
    Ok(PathBuf::from("cargo-commitlint"))
}

/// Quote a path for POSIX `sh` so it is treated as a literal.
///
/// Single quotes are the only construct that neutralizes every metacharacter
/// (`$`, backticks, `"`, globs). A literal single quote cannot appear inside a
/// single-quoted string, so it is emitted as `'\''` — close, escaped quote, reopen.
fn shell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', r"'\''"))
}

/// Render the `commit-msg` hook script that gets installed into `.git/hooks`.
///
/// NOTE: keep the validation invocation in sync with `.cargo-husky/hooks/commit-msg`,
/// which gates this repository's own commits. That script deliberately resolves the
/// binary differently (it searches `target/release` and `target/debug` first), but the
/// `check < "$COMMIT_MSG_FILE"` invocation and the stderr-directed skip warning must
/// stay the same in both. `test_husky_hook_and_generated_hook_agree_on_invocation`
/// enforces those shared invariants.
///
/// The resolved binary path is interpolated already single-quoted by `shell_quote`,
/// so the placeholders below must NOT be wrapped in quotes of their own. Only
/// `"$COMMIT_MSG_FILE"` stays double-quoted, because it has to expand.
fn generate_hook_script(binary_path: &Path) -> String {
    let quoted_path = shell_quote(binary_path);

    format!(
        r#"#!/bin/sh
# Git commit-msg hook installed by cargo-commitlint
# This hook validates commit messages according to Conventional Commits specification

COMMIT_MSG_FILE="$1"

if [ -x {bin_path} ]; then
    {bin_path} check < "$COMMIT_MSG_FILE" || exit 1
elif command -v cargo-commitlint >/dev/null 2>&1; then
    cargo-commitlint check < "$COMMIT_MSG_FILE" || exit 1
else
    echo "warning: cargo-commitlint not found; skipping commit message validation" >&2
    exit 0
fi
"#,
        bin_path = quoted_path
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a unique, empty hooks directory under the system temp dir.
    ///
    /// Avoids `env::set_current_dir` (global state) so tests stay parallel-safe.
    fn temp_hooks_dir(test_name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cargo-commitlint-{test_name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("failed to create temp hooks dir");
        dir
    }

    fn stub_binary() -> PathBuf {
        PathBuf::from("/usr/local/bin/cargo-commitlint")
    }

    #[test]
    fn test_hook_script_generation() {
        let path = Path::new("/usr/local/bin/cargo-commitlint");
        let script = generate_hook_script(path);
        // Marker string the uninstall safety check greps for
        assert!(script.contains("cargo-commitlint"));
        // Pinned binary is invoked directly, single-quoted, with a stdin redirect
        assert!(script.contains(r#"'/usr/local/bin/cargo-commitlint' check < "$COMMIT_MSG_FILE""#));
        // Executability guard for the pinned path
        assert!(script.contains(r#"[ -x '/usr/local/bin/cargo-commitlint' ]"#));
        // PATH fallback when the pinned path is not executable
        assert!(script.contains(r#"cargo-commitlint check < "$COMMIT_MSG_FILE""#));
        // The pinned path must never end up wrapped in double quotes again
        assert!(!script.contains(r#""/usr/local/bin/cargo-commitlint""#));
        // The fail-open skip warning must reach stderr, not stdout
        assert!(script.contains(
            r#"echo "warning: cargo-commitlint not found; skipping commit message validation" >&2"#
        ));
        // No cargo startup probe and no cat-pipe
        assert!(!script.contains("cargo commitlint --version"));
        assert!(!script.contains("cat "));
        assert!(script.starts_with("#!/bin/sh"));
    }

    #[test]
    fn test_hook_script_quotes_paths_with_spaces() {
        let path = Path::new("/opt/my tools/cargo-commitlint");
        let script = generate_hook_script(path);
        assert!(script.contains(r#"'/opt/my tools/cargo-commitlint' check < "$COMMIT_MSG_FILE""#));
        assert!(script.contains(r#"[ -x '/opt/my tools/cargo-commitlint' ]"#));
    }

    #[test]
    fn test_hook_script_neutralizes_shell_metacharacters() {
        // Command substitution in the checkout path must stay inert.
        let path = Path::new("/tmp/$(touch /tmp/pwned)/cargo-commitlint");
        let script = generate_hook_script(path);
        assert!(script.contains(r#"'/tmp/$(touch /tmp/pwned)/cargo-commitlint' check"#));
        assert!(script.contains(r#"[ -x '/tmp/$(touch /tmp/pwned)/cargo-commitlint' ]"#));
        // Every `$(` occurrence must sit inside the single-quoted literal.
        assert_eq!(script.matches("$(").count(), 2);
        assert!(!script.contains(r#""/tmp/$("#));

        // Backtick substitution likewise.
        let backtick = Path::new("/tmp/`touch /tmp/pwned`/cargo-commitlint");
        let script = generate_hook_script(backtick);
        assert!(script.contains(r#"'/tmp/`touch /tmp/pwned`/cargo-commitlint' check"#));
        assert!(script.contains(r#"[ -x '/tmp/`touch /tmp/pwned`/cargo-commitlint' ]"#));

        // An embedded double quote cannot break out of a single-quoted literal.
        let dquote = Path::new(r#"/tmp/a"; touch /tmp/pwned; "/cargo-commitlint"#);
        let script = generate_hook_script(dquote);
        assert!(script.contains(r#"'/tmp/a"; touch /tmp/pwned; "/cargo-commitlint' check"#));

        // The uninstall/backup marker survives via the PATH fallback and header.
        assert!(script.contains("cargo-commitlint"));
    }

    #[test]
    fn test_hook_script_escapes_embedded_single_quote() {
        let path = Path::new("/tmp/it's/cargo-commitlint");
        let script = generate_hook_script(path);
        assert!(script.contains(r"'/tmp/it'\''s/cargo-commitlint' check"));
        assert!(script.contains(r"[ -x '/tmp/it'\''s/cargo-commitlint' ]"));
        // Sanity: the raw, unescaped form must not appear.
        assert!(!script.contains(r"'/tmp/it's/cargo-commitlint'"));
    }

    /// End-to-end proof that the quoting holds at the shell level: the generated
    /// script must parse (`sh -n`) and must not run the command substitution that
    /// is embedded in the binary path when the hook is actually executed.
    #[cfg(unix)]
    #[test]
    fn test_generated_script_does_not_execute_path_substitution() {
        let dir = temp_hooks_dir("shell-metachar-exec");
        let canary = dir.join("pwned");
        let path = PathBuf::from(format!(
            "/nonexistent/$(touch {})/`touch {}`/cargo-commitlint",
            canary.display(),
            canary.display()
        ));

        let script_path = dir.join("commit-msg");
        fs::write(&script_path, generate_hook_script(&path)).unwrap();
        let msg_path = dir.join("COMMIT_EDITMSG");
        fs::write(&msg_path, "feat: subject\n").unwrap();

        // 1. The script is syntactically valid POSIX sh.
        let parsed = Command::new("sh")
            .arg("-n")
            .arg(&script_path)
            .output()
            .expect("failed to run sh -n");
        assert!(
            parsed.status.success(),
            "generated script failed `sh -n`: {}",
            String::from_utf8_lossy(&parsed.stderr)
        );

        // 2. Executing it with an empty PATH takes the fail-open branch (so the
        //    real binary is never invoked) but still evaluates `[ -x <path> ]`.
        //    `/bin/sh` is spelled out because an empty PATH also applies to the
        //    lookup of the interpreter itself.
        let ran = Command::new("/bin/sh")
            .arg(&script_path)
            .arg(&msg_path)
            .env("PATH", "")
            .output()
            .expect("failed to run generated hook");
        assert!(ran.status.success());

        // 3. Neither the `$(...)` nor the backtick substitution ever ran.
        assert!(
            !canary.exists(),
            "path substitution executed: {} was created",
            canary.display()
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_shell_quote_is_literal() {
        assert_eq!(shell_quote(Path::new("/a/b")), "'/a/b'");
        assert_eq!(shell_quote(Path::new("/a b/c")), "'/a b/c'");
        assert_eq!(shell_quote(Path::new("/a$b/c")), "'/a$b/c'");
        assert_eq!(shell_quote(Path::new("/it's")), r"'/it'\''s'");
    }

    #[test]
    fn test_install_backs_up_foreign_hook() {
        let hooks_dir = temp_hooks_dir("install-backs-up");
        let hook_path = hooks_dir.join("commit-msg");
        let foreign = "#!/bin/sh\necho other";
        fs::write(&hook_path, foreign).unwrap();

        install_to(&hooks_dir, &stub_binary()).unwrap();

        let backup = fs::read_to_string(hooks_dir.join("commit-msg.backup")).unwrap();
        assert_eq!(backup, foreign);
        assert!(fs::read_to_string(&hook_path)
            .unwrap()
            .contains("cargo-commitlint"));

        fs::remove_dir_all(&hooks_dir).unwrap();
    }

    #[test]
    fn test_install_does_not_backup_own_hook() {
        let hooks_dir = temp_hooks_dir("install-own-hook");
        let hook_path = hooks_dir.join("commit-msg");
        fs::write(
            &hook_path,
            "#!/bin/sh\n# installed by cargo-commitlint\nexit 0",
        )
        .unwrap();

        install_to(&hooks_dir, &stub_binary()).unwrap();

        assert!(!hooks_dir.join("commit-msg.backup").exists());
        let installed = fs::read_to_string(&hook_path).unwrap();
        assert!(installed.contains(r#"check < "$COMMIT_MSG_FILE""#));

        fs::remove_dir_all(&hooks_dir).unwrap();
    }

    #[test]
    fn test_install_does_not_clobber_existing_backup() {
        let hooks_dir = temp_hooks_dir("install-keeps-backup");
        let hook_path = hooks_dir.join("commit-msg");
        let foreign = "#!/bin/sh\necho other";
        let older_backup = "#!/bin/sh\necho older backup";
        fs::write(&hook_path, foreign).unwrap();
        fs::write(hooks_dir.join("commit-msg.backup"), older_backup).unwrap();

        install_to(&hooks_dir, &stub_binary()).unwrap();

        // The pre-existing backup survives untouched...
        assert_eq!(
            fs::read_to_string(hooks_dir.join("commit-msg.backup")).unwrap(),
            older_backup
        );
        // ...and the new backup lands at a suffixed name.
        assert_eq!(
            fs::read_to_string(hooks_dir.join("commit-msg.backup.1")).unwrap(),
            foreign
        );
        assert!(fs::read_to_string(&hook_path)
            .unwrap()
            .contains("cargo-commitlint"));

        fs::remove_dir_all(&hooks_dir).unwrap();
    }

    #[test]
    fn test_uninstall_removes_our_hook_and_restores_backup() {
        let hooks_dir = temp_hooks_dir("uninstall-restores");
        let hook_path = hooks_dir.join("commit-msg");
        let previous = "#!/bin/sh\necho previous hook";
        fs::write(&hook_path, generate_hook_script(&stub_binary())).unwrap();
        fs::write(hooks_dir.join("commit-msg.backup"), previous).unwrap();

        uninstall_from(&hooks_dir).unwrap();

        assert!(hook_path.exists());
        assert_eq!(fs::read_to_string(&hook_path).unwrap(), previous);
        assert!(!hooks_dir.join("commit-msg.backup").exists());

        fs::remove_dir_all(&hooks_dir).unwrap();
    }

    #[test]
    fn test_uninstall_preserves_foreign_hook() {
        let hooks_dir = temp_hooks_dir("uninstall-preserves");
        let hook_path = hooks_dir.join("commit-msg");
        let foreign = "#!/bin/sh\necho other";
        fs::write(&hook_path, foreign).unwrap();

        uninstall_from(&hooks_dir).unwrap();

        assert!(hook_path.exists());
        assert_eq!(fs::read_to_string(&hook_path).unwrap(), foreign);

        fs::remove_dir_all(&hooks_dir).unwrap();
    }

    #[test]
    fn test_husky_hook_and_generated_hook_agree_on_invocation() {
        // `.cargo-husky` is a dev-only concern and is excluded from the published
        // package, so skip this check when the source tree isn't available.
        let husky_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".cargo-husky")
            .join("hooks")
            .join("commit-msg");
        if !husky_path.exists() {
            return;
        }

        let husky = fs::read_to_string(&husky_path).expect("failed to read husky commit-msg hook");
        let generated = generate_hook_script(&stub_binary());

        for (label, script) in [("husky", husky.as_str()), ("generated", generated.as_str())] {
            assert!(
                script.contains(r#"check < "$COMMIT_MSG_FILE""#),
                "{label} hook must invoke `check < \"$COMMIT_MSG_FILE\"`"
            );
            assert!(
                script.contains(">&2"),
                "{label} hook must send its skip warning to stderr"
            );
            assert!(
                !script.contains("cargo commitlint --version"),
                "{label} hook must not probe the binary with `cargo commitlint --version`"
            );
            assert!(
                !script.contains(r#"cat "$COMMIT_MSG_FILE" |"#),
                "{label} hook must not pipe the commit message through cat"
            );
        }
    }
}
