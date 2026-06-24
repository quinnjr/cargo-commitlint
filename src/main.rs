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

fn main() {
    // Filter out "commitlint" argument if passed by cargo
    let args: Vec<String> = std::env::args().filter(|arg| arg != "commitlint").collect();

    let cli = Cli::parse_from(args);

    let result = match cli.command {
        Commands::Install => {
            hook::HookInstaller::install().map_err(|e| format!("Failed to install hook: {}", e))
        }
        Commands::Uninstall => {
            hook::HookInstaller::uninstall().map_err(|e| format!("Failed to uninstall hook: {}", e))
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
        config::Config::from_file(&path)
            .map_err(|e| format!("Failed to load config from {}: {}", path.display(), e))?
    } else {
        config::Config::from_default_locations()
            .map_err(|e| format!("Failed to load config: {}", e))?
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
