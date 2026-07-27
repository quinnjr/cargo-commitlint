mod commit;
mod config;
mod hook;
mod validator;

use clap::{Parser, Subcommand};
use std::io::{self, Read};
use std::process;

#[derive(Parser)]
#[command(name = "cargo-commitlint")]
#[command(bin_name = "cargo commitlint")]
#[command(
    about = "A Rust-based commit message linter following Conventional Commits specification"
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install git hook for commit message validation
    Install,
    /// Uninstall git hook
    Uninstall,
    /// Validate a commit message
    Check {
        /// Commit message to validate (if not provided, reads from stdin)
        #[arg(short, long)]
        message: Option<String>,
        /// Path to configuration file
        #[arg(short, long)]
        config: Option<std::path::PathBuf>,
    },
}

/// Strip the `commitlint` token that cargo injects when invoked as `cargo commitlint`.
/// Only the subcommand position is removed, so a message that happens to contain
/// "commitlint" survives intact.
fn strip_cargo_subcommand(mut args: Vec<String>) -> Vec<String> {
    if args.get(1).map(String::as_str) == Some("commitlint") {
        args.remove(1);
    }
    args
}

fn main() {
    let args = strip_cargo_subcommand(std::env::args().collect());

    let cli = Cli::parse_from(args);

    let result = match cli.command {
        Commands::Install => hook::install().map_err(|e| format!("Failed to install hook: {}", e)),
        Commands::Uninstall => {
            hook::uninstall().map_err(|e| format!("Failed to uninstall hook: {}", e))
        }
        Commands::Check { message, config } => validate_commit_message(message, config),
    };

    match result {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn validate_commit_message(
    message: Option<String>,
    config_path: Option<std::path::PathBuf>,
) -> Result<(), String> {
    // Load configuration
    let config = if let Some(path) = config_path {
        let config = config::Config::from_file(&path)
            .map_err(|e| format!("Failed to load config from {}: {}", path.display(), e))?;
        config
            .validate()
            .map_err(|e| format!("Invalid config {}: {}", path.display(), e))?;
        config
    } else {
        let config = config::Config::from_default_locations()
            .map_err(|e| format!("Failed to load config: {}", e))?;
        config
            .validate()
            .map_err(|e| format!("Invalid config: {}", e))?;
        config
    };

    // Get commit message
    let commit_msg = if let Some(msg) = message {
        msg
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read from stdin: {}", e))?;
        buffer
    };

    // Validate
    let validator = validator::Validator::new(config);
    match validator.validate(&commit_msg) {
        Ok(()) => {
            println!("✓ Commit message is valid");
            Ok(())
        }
        Err(errors) => {
            eprintln!("✗ Commit message validation failed:\n");
            for error in errors {
                eprintln!("  - [{}] {}", error.rule, error.message);
            }
            Err("Validation failed".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strip_cargo_subcommand;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn strips_cargo_injected_subcommand_token() {
        assert_eq!(
            strip_cargo_subcommand(argv(&["cargo-commitlint", "commitlint", "check"])),
            argv(&["cargo-commitlint", "check"])
        );
    }

    #[test]
    fn preserves_commitlint_inside_a_commit_message() {
        let args = argv(&[
            "cargo-commitlint",
            "check",
            "--message",
            "fix commitlint bug",
        ]);
        assert_eq!(strip_cargo_subcommand(args.clone()), args);
    }

    #[test]
    fn leaves_direct_invocation_untouched() {
        let args = argv(&["cargo-commitlint", "check"]);
        assert_eq!(strip_cargo_subcommand(args.clone()), args);
    }

    #[test]
    fn handles_argv_with_only_the_program_name() {
        let args = argv(&["cargo-commitlint"]);
        assert_eq!(strip_cargo_subcommand(args.clone()), args);
    }
}
