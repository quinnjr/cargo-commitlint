//! cargo-commitlint - A Rust-based commit message linter
//!
//! Full compatibility with commitlint (Node.js) configuration and rules.

mod commit;
mod config;
mod format;
mod git;
mod hook;
mod rules;
mod validator;

use clap::{Parser, Subcommand};
use colored::Colorize;
use config::Config;
use format::{Formatter, OutputFormat};
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use validator::Validator;

#[derive(Parser)]
#[command(name = "cargo-commitlint")]
#[command(bin_name = "cargo commitlint")]
#[command(about = "Lint commit messages according to Conventional Commits specification")]
#[command(version)]
#[command(after_help = "Examples:
  cargo commitlint check --message \"feat: add new feature\"
  cargo commitlint check --edit
  cargo commitlint check --from HEAD~5 --to HEAD
  echo \"feat: test\" | cargo commitlint check
  cargo commitlint install")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // Global options that can be used without subcommand for compatibility
    /// Path to configuration file
    #[arg(short = 'g', long, global = true)]
    config: Option<PathBuf>,

    /// Directory to execute in
    #[arg(short = 'd', long, global = true)]
    cwd: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint commit messages (default command)
    #[command(alias = "lint")]
    Check {
        /// Commit message to validate (if not provided, reads from stdin)
        #[arg(short, long)]
        message: Option<String>,

        /// Read commit message from the specified file or .git/COMMIT_EDITMSG
        #[arg(short, long, value_name = "FILE", num_args = 0..=1, default_missing_value = "")]
        edit: Option<PathBuf>,

        /// Read message from file at path given by environment variable
        #[arg(short = 'E', long, value_name = "VAR")]
        env: Option<String>,

        /// Lower end of commit range to lint (exclusive)
        #[arg(short, long)]
        from: Option<String>,

        /// Upper end of commit range to lint (inclusive)
        #[arg(short, long)]
        to: Option<String>,

        /// Use last tag as lower end of commit range
        #[arg(long)]
        from_last_tag: bool,

        /// Lint only the last commit
        #[arg(short, long)]
        last: bool,

        /// Output format (text, json, compact)
        #[arg(short = 'o', long, value_name = "FORMAT")]
        format: Option<String>,

        /// Enable/disable colored output
        #[arg(short = 'c', long, default_value = "true")]
        color: bool,

        /// Suppress output on success
        #[arg(short, long)]
        quiet: bool,

        /// Show output for valid commits
        #[arg(short = 'V', long)]
        verbose: bool,

        /// Strict mode: exit 2 for warnings, 3 for errors
        #[arg(short, long)]
        strict: bool,

        /// Help URL to display in error messages
        #[arg(short = 'H', long)]
        help_url: Option<String>,

        /// Additional git log arguments
        #[arg(long, value_name = "ARGS")]
        git_log_args: Option<String>,
    },

    /// Print resolved configuration
    PrintConfig {
        /// Output format (text, json)
        #[arg(short = 'o', long, default_value = "json")]
        format: String,
    },

    /// Install git commit-msg hook
    Install {
        /// Force overwrite existing hook
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall git commit-msg hook
    Uninstall,
}

fn main() {
    // Filter out "commitlint" argument if passed by cargo
    let args: Vec<String> = std::env::args()
        .enumerate()
        .filter(|(i, arg)| !(*i == 1 && arg == "commitlint"))
        .map(|(_, arg)| arg)
        .collect();

    let cli = Cli::parse_from(&args);

    // Change to specified directory if provided
    if let Some(ref cwd) = cli.cwd {
        if let Err(e) = std::env::set_current_dir(cwd) {
            eprintln!("Error: Failed to change directory to {}: {}", cwd.display(), e);
            process::exit(1);
        }
    }

    let result = match cli.command {
        Some(Commands::Check {
            message,
            edit,
            env,
            from,
            to,
            from_last_tag,
            last,
            format,
            color,
            quiet,
            verbose,
            strict,
            help_url,
            git_log_args: _,
        }) => {
            run_check(CheckOptions {
                message,
                edit,
                env,
                from,
                to,
                from_last_tag,
                last,
                format,
                color,
                quiet,
                verbose,
                strict,
                help_url,
                config_path: cli.config,
            })
        }
        Some(Commands::PrintConfig { format }) => {
            run_print_config(&format, cli.config)
        }
        Some(Commands::Install { force }) => {
            hook::HookInstaller::install(force)
                .map(|_| 0)
                .map_err(|e| format!("Failed to install hook: {}", e))
        }
        Some(Commands::Uninstall) => {
            hook::HookInstaller::uninstall()
                .map(|_| 0)
                .map_err(|e| format!("Failed to uninstall hook: {}", e))
        }
        None => {
            // Default to check with stdin
            run_check(CheckOptions {
                message: None,
                edit: None,
                env: None,
                from: None,
                to: None,
                from_last_tag: false,
                last: false,
                format: None,
                color: true,
                quiet: false,
                verbose: false,
                strict: false,
                help_url: None,
                config_path: cli.config,
            })
        }
    };

    match result {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e);
            process::exit(1);
        }
    }
}

struct CheckOptions {
    message: Option<String>,
    edit: Option<PathBuf>,
    env: Option<String>,
    from: Option<String>,
    to: Option<String>,
    from_last_tag: bool,
    last: bool,
    format: Option<String>,
    color: bool,
    quiet: bool,
    verbose: bool,
    strict: bool,
    help_url: Option<String>,
    config_path: Option<PathBuf>,
}

fn run_check(opts: CheckOptions) -> Result<i32, String> {
    // Load configuration
    let config = load_config(opts.config_path.as_deref())?;

    // Determine output format
    let output_format = opts
        .format
        .as_deref()
        .map(|f| f.parse::<OutputFormat>())
        .transpose()
        .map_err(|e| e)?
        .unwrap_or_default();

    // Get help URL from config or CLI
    let help_url = opts.help_url.clone().or_else(|| config.help_url.clone());

    // Create formatter
    let formatter = Formatter::new(output_format, opts.color, opts.verbose, help_url);

    // Get commit messages to validate
    let messages = get_commit_messages(&opts, &config)?;

    if messages.is_empty() {
        if !opts.quiet {
            eprintln!("{}", "No commits to lint".yellow());
        }
        return Ok(0);
    }

    // Validate all messages
    let validator = Validator::new(config.clone());
    let mut results = Vec::new();
    let mut has_errors = false;
    let mut has_warnings = false;

    for message in &messages {
        // Check if should be ignored
        if config.should_ignore(message) {
            continue;
        }

        let result = validator.validate(message);

        if !result.valid {
            has_errors = true;
        }
        if result.warnings > 0 {
            has_warnings = true;
        }

        results.push(result);
    }

    // Output results
    if !opts.quiet || has_errors || has_warnings {
        for result in &results {
            if !opts.quiet || !result.valid || opts.verbose {
                println!("{}", formatter.format_result(result));
            }
        }

        // Summary for multiple commits
        if results.len() > 1 {
            format::print_summary(&results, opts.color);
        }
    }

    // Determine exit code
    if has_errors {
        if opts.strict {
            Ok(3)
        } else {
            Ok(1)
        }
    } else if has_warnings && opts.strict {
        Ok(2)
    } else {
        Ok(0)
    }
}

fn get_commit_messages(opts: &CheckOptions, _config: &Config) -> Result<Vec<String>, String> {
    // Priority order:
    // 1. --message flag
    // 2. --edit flag (read from file)
    // 3. --env flag (read from env var path)
    // 4. --from/--to or --last or --from-last-tag (git log)
    // 5. stdin

    // Direct message
    if let Some(ref msg) = opts.message {
        return Ok(vec![msg.clone()]);
    }

    // Read from file (--edit)
    if let Some(ref edit) = opts.edit {
        let path = if edit.as_os_str().is_empty() {
            // --edit without value: default to .git/COMMIT_EDITMSG
            let cwd = std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;
            git::find_git_dir(&cwd)
                .map(|g| g.join("COMMIT_EDITMSG"))
                .ok_or_else(|| "Not in a git repository".to_string())?
        } else {
            // --edit with value: use the provided path
            edit.clone()
        };

        let content = git::read_commit_message_file(&path)
            .map_err(|e| format!("Failed to read commit message: {}", e))?;
        return Ok(vec![content]);
    }

    // Read from env var path
    if let Some(ref var) = opts.env {
        let path = std::env::var(var)
            .map_err(|_| format!("Environment variable '{}' not set", var))?;
        let content = git::read_commit_message_file(&PathBuf::from(&path))
            .map_err(|e| format!("Failed to read commit message from {}: {}", path, e))?;
        return Ok(vec![content]);
    }

    // Git log based options
    if opts.last || opts.from.is_some() || opts.to.is_some() || opts.from_last_tag {
        let repo = git::GitRepo::open_current()
            .map_err(|e| format!("Failed to open git repository: {}", e))?;

        if opts.last {
            return repo
                .get_last_n_commits(1)
                .map_err(|e| format!("Failed to get last commit: {}", e));
        }

        if opts.from_last_tag {
            return repo
                .get_commits_since_last_tag()
                .map_err(|e| format!("Failed to get commits since last tag: {}", e));
        }

        return repo
            .get_commits_in_range(opts.from.as_deref(), opts.to.as_deref())
            .map_err(|e| format!("Failed to get commits in range: {}", e));
    }

    // Read from stdin
    if atty::is(atty::Stream::Stdin) {
        // No stdin input available
        return Err("No commit message provided. Use --message, --edit, or pipe a message to stdin.".to_string());
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| format!("Failed to read from stdin: {}", e))?;

    if buffer.trim().is_empty() {
        return Err("Empty commit message".to_string());
    }

    Ok(vec![buffer])
}

fn load_config(config_path: Option<&std::path::Path>) -> Result<Config, String> {
    if let Some(path) = config_path {
        Config::from_file(path)
            .map_err(|e| format!("Failed to load config from {}: {}", path.display(), e))
    } else {
        Config::from_default_locations()
            .map_err(|e| format!("Failed to load config: {}", e))
    }
}

fn run_print_config(format: &str, config_path: Option<PathBuf>) -> Result<i32, String> {
    let config = load_config(config_path.as_deref())?;

    let output = match format.to_lowercase().as_str() {
        "json" => config.to_json().map_err(|e| e.to_string())?,
        "text" | "toml" => config.to_toml().map_err(|e| e.to_string())?,
        _ => return Err(format!("Unknown format: {}. Use 'json' or 'text'", format)),
    };

    println!("{}", output);
    Ok(0)
}
