//! Validator module implementing all commitlint rule validations
//!
//! Each rule follows the pattern:
//! - Level 0: disabled
//! - Level 1: warning
//! - Level 2: error
//! - Applicable::Always: rule must be satisfied
//! - Applicable::Never: rule must NOT be satisfied (inverted)

use crate::commit::ConventionalCommit;
use crate::config::Config;
use crate::rules::{Applicable, RuleLevel};
use colored::Colorize;

/// Result of validating a single rule
#[derive(Debug, Clone)]
pub struct RuleResult {
    /// Rule name (e.g., "type-enum")
    pub name: String,
    /// Whether the rule passed
    pub valid: bool,
    /// Severity level
    pub level: RuleLevel,
    /// Human-readable message
    pub message: String,
}

impl RuleResult {
    fn error(name: &str, message: String) -> Self {
        Self {
            name: name.to_string(),
            valid: false,
            level: RuleLevel::Error,
            message,
        }
    }

    fn warning(name: &str, message: String) -> Self {
        Self {
            name: name.to_string(),
            valid: false,
            level: RuleLevel::Warning,
            message,
        }
    }

    fn pass(name: &str) -> Self {
        Self {
            name: name.to_string(),
            valid: true,
            level: RuleLevel::Disabled,
            message: String::new(),
        }
    }
}

/// Result of validating a commit
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// The original commit message
    pub input: String,
    /// Parsed commit
    pub commit: ConventionalCommit,
    /// All rule results
    pub results: Vec<RuleResult>,
    /// Whether the commit is valid (no errors)
    pub valid: bool,
    /// Number of errors
    pub errors: usize,
    /// Number of warnings
    pub warnings: usize,
}

impl ValidationResult {
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.warnings > 0
    }

    /// Get all failed rules
    pub fn failures(&self) -> Vec<&RuleResult> {
        self.results.iter().filter(|r| !r.valid).collect()
    }

    /// Format the result for display
    pub fn format(&self, color: bool, verbose: bool) -> String {
        let mut output = String::new();

        // Header
        let header_display = if self.commit.header.len() > 72 {
            format!("{}...", &self.commit.header[..69])
        } else {
            self.commit.header.clone()
        };

        if color {
            output.push_str(&format!("⧗   input: {}\n", header_display.dimmed()));
        } else {
            output.push_str(&format!("⧗   input: {}\n", header_display));
        }

        // Results
        if self.valid && !verbose {
            if color {
                output.push_str(&format!("{}  found 0 problems, 0 warnings\n", "✔".green()));
            } else {
                output.push_str("✔  found 0 problems, 0 warnings\n");
            }
        } else {
            let problems: Vec<&RuleResult> = self.results
                .iter()
                .filter(|r| !r.valid)
                .collect();

            for result in &problems {
                let symbol = match result.level {
                    RuleLevel::Error => if color { "✖".red().to_string() } else { "✖".to_string() },
                    RuleLevel::Warning => if color { "⚠".yellow().to_string() } else { "⚠".to_string() },
                    RuleLevel::Disabled => continue,
                };

                let level_str = match result.level {
                    RuleLevel::Error => "error",
                    RuleLevel::Warning => "warning",
                    RuleLevel::Disabled => continue,
                };

                if color {
                    output.push_str(&format!(
                        "{}   {} {} [{}]\n",
                        symbol,
                        result.message,
                        format!("[{}]", level_str).dimmed(),
                        result.name.cyan()
                    ));
                } else {
                    output.push_str(&format!(
                        "{}   {} [{}] [{}]\n",
                        symbol,
                        result.message,
                        level_str,
                        result.name
                    ));
                }
            }

            // Summary
            if color {
                let status = if self.errors > 0 { "✖".red() } else { "⚠".yellow() };
                output.push_str(&format!(
                    "\n{}  found {} problems, {} warnings\n",
                    status,
                    self.errors.to_string().red(),
                    self.warnings.to_string().yellow()
                ));
            } else {
                let status = if self.errors > 0 { "✖" } else { "⚠" };
                output.push_str(&format!(
                    "\n{}  found {} problems, {} warnings\n",
                    status, self.errors, self.warnings
                ));
            }
        }

        output
    }

    /// Format as JSON
    pub fn to_json(&self) -> String {
        serde_json::json!({
            "valid": self.valid,
            "errors": self.results.iter()
                .filter(|r| !r.valid && r.level == RuleLevel::Error)
                .map(|r| serde_json::json!({
                    "name": r.name,
                    "message": r.message,
                    "level": 2
                }))
                .collect::<Vec<_>>(),
            "warnings": self.results.iter()
                .filter(|r| !r.valid && r.level == RuleLevel::Warning)
                .map(|r| serde_json::json!({
                    "name": r.name,
                    "message": r.message,
                    "level": 1
                }))
                .collect::<Vec<_>>(),
            "input": self.input,
        }).to_string()
    }
}

/// The main validator
pub struct Validator {
    config: Config,
}

impl Validator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Validate a commit message
    pub fn validate(&self, message: &str) -> ValidationResult {
        let commit = ConventionalCommit::parse(message, &self.config.parser.header_pattern);

        let mut results = Vec::new();

        // Header rules
        results.push(self.validate_header_max_length(&commit));
        results.push(self.validate_header_min_length(&commit));
        results.push(self.validate_header_case(&commit));
        results.push(self.validate_header_full_stop(&commit));
        results.push(self.validate_header_trim(&commit));

        // Type rules
        results.push(self.validate_type_enum(&commit));
        results.push(self.validate_type_case(&commit));
        results.push(self.validate_type_empty(&commit));
        results.push(self.validate_type_max_length(&commit));
        results.push(self.validate_type_min_length(&commit));

        // Scope rules
        results.push(self.validate_scope_enum(&commit));
        results.push(self.validate_scope_case(&commit));
        results.push(self.validate_scope_empty(&commit));
        results.push(self.validate_scope_max_length(&commit));
        results.push(self.validate_scope_min_length(&commit));

        // Subject rules
        results.push(self.validate_subject_case(&commit));
        results.push(self.validate_subject_empty(&commit));
        results.push(self.validate_subject_full_stop(&commit));
        results.push(self.validate_subject_max_length(&commit));
        results.push(self.validate_subject_min_length(&commit));
        results.push(self.validate_subject_exclamation_mark(&commit));

        // Body rules
        results.push(self.validate_body_case(&commit));
        results.push(self.validate_body_empty(&commit));
        results.push(self.validate_body_full_stop(&commit));
        results.push(self.validate_body_leading_blank(&commit));
        results.push(self.validate_body_max_length(&commit));
        results.push(self.validate_body_max_line_length(&commit));
        results.push(self.validate_body_min_length(&commit));

        // Footer rules
        results.push(self.validate_footer_empty(&commit));
        results.push(self.validate_footer_leading_blank(&commit));
        results.push(self.validate_footer_max_length(&commit));
        results.push(self.validate_footer_max_line_length(&commit));
        results.push(self.validate_footer_min_length(&commit));

        // Other rules
        results.push(self.validate_references_empty(&commit));
        results.push(self.validate_signed_off_by(&commit));
        results.push(self.validate_trailer_exists(&commit));

        // Filter out disabled rules
        let active_results: Vec<RuleResult> = results
            .into_iter()
            .filter(|r| r.level != RuleLevel::Disabled || !r.valid)
            .collect();

        let errors = active_results.iter().filter(|r| !r.valid && r.level == RuleLevel::Error).count();
        let warnings = active_results.iter().filter(|r| !r.valid && r.level == RuleLevel::Warning).count();

        ValidationResult {
            input: message.to_string(),
            commit,
            results: active_results,
            valid: errors == 0,
            errors,
            warnings,
        }
    }

    // ==================== Header Rules ====================

    fn validate_header_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.header_max_length;
        if !rule.is_active() {
            return RuleResult::pass("header-max-length");
        }

        let len = commit.header.chars().count();
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("header-max-length")
        } else {
            let msg = format!("header must not be longer than {} characters, current length is {}", max, len);
            match rule.level {
                RuleLevel::Error => RuleResult::error("header-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("header-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("header-max-length"),
            }
        }
    }

    fn validate_header_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.header_min_length;
        if !rule.is_active() {
            return RuleResult::pass("header-min-length");
        }

        let len = commit.header.chars().count();
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("header-min-length")
        } else {
            let msg = format!("header must be at least {} characters, current length is {}", min, len);
            match rule.level {
                RuleLevel::Error => RuleResult::error("header-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("header-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("header-min-length"),
            }
        }
    }

    fn validate_header_case(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.header_case;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("header-case");
        }

        let passes = rule.value.iter().any(|case| case.validate(&commit.header));

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("header-case")
        } else {
            let cases: Vec<&str> = rule.value.iter().map(|c| c.as_str()).collect();
            let msg = format!("header must be {}", cases.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("header-case", msg),
                RuleLevel::Warning => RuleResult::warning("header-case", msg),
                RuleLevel::Disabled => RuleResult::pass("header-case"),
            }
        }
    }

    fn validate_header_full_stop(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.header_full_stop;
        if !rule.is_active() {
            return RuleResult::pass("header-full-stop");
        }

        let char = &rule.value;
        let ends_with = commit.header.ends_with(char);

        let result = match rule.applicable {
            Applicable::Always => ends_with,
            Applicable::Never => !ends_with,
        };

        if result {
            RuleResult::pass("header-full-stop")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => format!("header must end with '{}'", char),
                Applicable::Never => format!("header must not end with '{}'", char),
            };
            match rule.level {
                RuleLevel::Error => RuleResult::error("header-full-stop", msg),
                RuleLevel::Warning => RuleResult::warning("header-full-stop", msg),
                RuleLevel::Disabled => RuleResult::pass("header-full-stop"),
            }
        }
    }

    fn validate_header_trim(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.header_trim;
        if !rule.is_active() {
            return RuleResult::pass("header-trim");
        }

        let trimmed = commit.header.trim();
        let is_trimmed = commit.header == trimmed;

        let result = match rule.applicable {
            Applicable::Always => is_trimmed,
            Applicable::Never => !is_trimmed,
        };

        if result {
            RuleResult::pass("header-trim")
        } else {
            let msg = "header must not have leading or trailing whitespace".to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("header-trim", msg),
                RuleLevel::Warning => RuleResult::warning("header-trim", msg),
                RuleLevel::Disabled => RuleResult::pass("header-trim"),
            }
        }
    }

    // ==================== Type Rules ====================

    fn validate_type_enum(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.type_enum;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("type-enum");
        }

        let commit_type = match &commit.commit_type {
            Some(t) => t,
            None => return RuleResult::error("type-enum", "type may not be empty".to_string()),
        };

        let in_enum = rule.value.contains(commit_type);

        let result = match rule.applicable {
            Applicable::Always => in_enum,
            Applicable::Never => !in_enum,
        };

        if result {
            RuleResult::pass("type-enum")
        } else {
            let msg = format!("type must be one of [{}]", rule.value.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("type-enum", msg),
                RuleLevel::Warning => RuleResult::warning("type-enum", msg),
                RuleLevel::Disabled => RuleResult::pass("type-enum"),
            }
        }
    }

    fn validate_type_case(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.type_case;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("type-case");
        }

        let commit_type = match &commit.commit_type {
            Some(t) => t,
            None => return RuleResult::pass("type-case"),
        };

        let passes = rule.value.iter().any(|case| case.validate(commit_type));

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("type-case")
        } else {
            let cases: Vec<&str> = rule.value.iter().map(|c| c.as_str()).collect();
            let msg = format!("type must be {}", cases.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("type-case", msg),
                RuleLevel::Warning => RuleResult::warning("type-case", msg),
                RuleLevel::Disabled => RuleResult::pass("type-case"),
            }
        }
    }

    fn validate_type_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.type_empty;
        if !rule.is_active() {
            return RuleResult::pass("type-empty");
        }

        let is_empty = commit.commit_type.as_ref().map(|t| t.is_empty()).unwrap_or(true);

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("type-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "type must be empty",
                Applicable::Never => "type may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("type-empty", msg),
                RuleLevel::Warning => RuleResult::warning("type-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("type-empty"),
            }
        }
    }

    fn validate_type_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.type_max_length;
        if !rule.is_active() {
            return RuleResult::pass("type-max-length");
        }

        let len = commit.commit_type.as_ref().map(|t| t.chars().count()).unwrap_or(0);
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("type-max-length")
        } else {
            let msg = format!("type must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("type-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("type-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("type-max-length"),
            }
        }
    }

    fn validate_type_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.type_min_length;
        if !rule.is_active() {
            return RuleResult::pass("type-min-length");
        }

        let len = commit.commit_type.as_ref().map(|t| t.chars().count()).unwrap_or(0);
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("type-min-length")
        } else {
            let msg = format!("type must be at least {} characters", min);
            match rule.level {
                RuleLevel::Error => RuleResult::error("type-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("type-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("type-min-length"),
            }
        }
    }

    // ==================== Scope Rules ====================

    fn validate_scope_enum(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.scope_enum;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("scope-enum");
        }

        let scope = match &commit.scope {
            Some(s) => s,
            None => return RuleResult::pass("scope-enum"), // No scope is OK if scope-empty allows it
        };

        let in_enum = rule.value.contains(scope);

        let result = match rule.applicable {
            Applicable::Always => in_enum,
            Applicable::Never => !in_enum,
        };

        if result {
            RuleResult::pass("scope-enum")
        } else {
            let msg = format!("scope must be one of [{}]", rule.value.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("scope-enum", msg),
                RuleLevel::Warning => RuleResult::warning("scope-enum", msg),
                RuleLevel::Disabled => RuleResult::pass("scope-enum"),
            }
        }
    }

    fn validate_scope_case(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.scope_case;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("scope-case");
        }

        let scope = match &commit.scope {
            Some(s) => s,
            None => return RuleResult::pass("scope-case"),
        };

        let passes = rule.value.iter().any(|case| case.validate(scope));

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("scope-case")
        } else {
            let cases: Vec<&str> = rule.value.iter().map(|c| c.as_str()).collect();
            let msg = format!("scope must be {}", cases.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("scope-case", msg),
                RuleLevel::Warning => RuleResult::warning("scope-case", msg),
                RuleLevel::Disabled => RuleResult::pass("scope-case"),
            }
        }
    }

    fn validate_scope_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.scope_empty;
        if !rule.is_active() {
            return RuleResult::pass("scope-empty");
        }

        let is_empty = commit.scope.as_ref().map(|s| s.is_empty()).unwrap_or(true);

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("scope-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "scope must be empty",
                Applicable::Never => "scope may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("scope-empty", msg),
                RuleLevel::Warning => RuleResult::warning("scope-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("scope-empty"),
            }
        }
    }

    fn validate_scope_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.scope_max_length;
        if !rule.is_active() {
            return RuleResult::pass("scope-max-length");
        }

        let len = commit.scope.as_ref().map(|s| s.chars().count()).unwrap_or(0);
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("scope-max-length")
        } else {
            let msg = format!("scope must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("scope-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("scope-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("scope-max-length"),
            }
        }
    }

    fn validate_scope_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.scope_min_length;
        if !rule.is_active() {
            return RuleResult::pass("scope-min-length");
        }

        // Only validate if there is a scope
        let scope = match &commit.scope {
            Some(s) => s,
            None => return RuleResult::pass("scope-min-length"),
        };

        let len = scope.chars().count();
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("scope-min-length")
        } else {
            let msg = format!("scope must be at least {} characters", min);
            match rule.level {
                RuleLevel::Error => RuleResult::error("scope-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("scope-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("scope-min-length"),
            }
        }
    }

    // ==================== Subject Rules ====================

    fn validate_subject_case(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_case;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("subject-case");
        }

        let subject = match &commit.subject {
            Some(s) => s.trim(),
            None => return RuleResult::pass("subject-case"),
        };

        if subject.is_empty() {
            return RuleResult::pass("subject-case");
        }

        let passes = rule.value.iter().any(|case| case.validate(subject));

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("subject-case")
        } else {
            let cases: Vec<&str> = rule.value.iter().map(|c| c.as_str()).collect();
            let msg = format!("subject must be {}", cases.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-case", msg),
                RuleLevel::Warning => RuleResult::warning("subject-case", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-case"),
            }
        }
    }

    fn validate_subject_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_empty;
        if !rule.is_active() {
            return RuleResult::pass("subject-empty");
        }

        let is_empty = commit.subject.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true);

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("subject-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "subject must be empty",
                Applicable::Never => "subject may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-empty", msg),
                RuleLevel::Warning => RuleResult::warning("subject-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-empty"),
            }
        }
    }

    fn validate_subject_full_stop(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_full_stop;
        if !rule.is_active() {
            return RuleResult::pass("subject-full-stop");
        }

        let subject = match &commit.subject {
            Some(s) => s,
            None => return RuleResult::pass("subject-full-stop"),
        };

        let char = &rule.value;
        let ends_with = subject.ends_with(char);

        let result = match rule.applicable {
            Applicable::Always => ends_with,
            Applicable::Never => !ends_with,
        };

        if result {
            RuleResult::pass("subject-full-stop")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => format!("subject must end with '{}'", char),
                Applicable::Never => format!("subject may not end with '{}'", char),
            };
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-full-stop", msg),
                RuleLevel::Warning => RuleResult::warning("subject-full-stop", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-full-stop"),
            }
        }
    }

    fn validate_subject_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_max_length;
        if !rule.is_active() {
            return RuleResult::pass("subject-max-length");
        }

        let len = commit.subject.as_ref().map(|s| s.chars().count()).unwrap_or(0);
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("subject-max-length")
        } else {
            let msg = format!("subject must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("subject-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-max-length"),
            }
        }
    }

    fn validate_subject_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_min_length;
        if !rule.is_active() {
            return RuleResult::pass("subject-min-length");
        }

        let len = commit.subject.as_ref().map(|s| s.chars().count()).unwrap_or(0);
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("subject-min-length")
        } else {
            let msg = format!("subject must be at least {} characters", min);
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("subject-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-min-length"),
            }
        }
    }

    fn validate_subject_exclamation_mark(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.subject_exclamation_mark;
        if !rule.is_active() {
            return RuleResult::pass("subject-exclamation-mark");
        }

        // Check if there's a ! before the colon (breaking change marker)
        let has_exclamation = commit.breaking;

        let result = match rule.applicable {
            Applicable::Always => has_exclamation,
            Applicable::Never => !has_exclamation,
        };

        if result {
            RuleResult::pass("subject-exclamation-mark")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "subject must have exclamation mark for breaking changes",
                Applicable::Never => "subject must not have exclamation mark",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("subject-exclamation-mark", msg),
                RuleLevel::Warning => RuleResult::warning("subject-exclamation-mark", msg),
                RuleLevel::Disabled => RuleResult::pass("subject-exclamation-mark"),
            }
        }
    }

    // ==================== Body Rules ====================

    fn validate_body_case(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_case;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("body-case");
        }

        let body = match &commit.body {
            Some(b) => b,
            None => return RuleResult::pass("body-case"),
        };

        let passes = rule.value.iter().any(|case| case.validate(body));

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("body-case")
        } else {
            let cases: Vec<&str> = rule.value.iter().map(|c| c.as_str()).collect();
            let msg = format!("body must be {}", cases.join(", "));
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-case", msg),
                RuleLevel::Warning => RuleResult::warning("body-case", msg),
                RuleLevel::Disabled => RuleResult::pass("body-case"),
            }
        }
    }

    fn validate_body_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_empty;
        if !rule.is_active() {
            return RuleResult::pass("body-empty");
        }

        let is_empty = commit.body.as_ref().map(|b| b.trim().is_empty()).unwrap_or(true);

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("body-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "body must be empty",
                Applicable::Never => "body may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-empty", msg),
                RuleLevel::Warning => RuleResult::warning("body-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("body-empty"),
            }
        }
    }

    fn validate_body_full_stop(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_full_stop;
        if !rule.is_active() {
            return RuleResult::pass("body-full-stop");
        }

        let body = match &commit.body {
            Some(b) => b,
            None => return RuleResult::pass("body-full-stop"),
        };

        let char = &rule.value;
        let ends_with = body.trim_end().ends_with(char);

        let result = match rule.applicable {
            Applicable::Always => ends_with,
            Applicable::Never => !ends_with,
        };

        if result {
            RuleResult::pass("body-full-stop")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => format!("body must end with '{}'", char),
                Applicable::Never => format!("body may not end with '{}'", char),
            };
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-full-stop", msg),
                RuleLevel::Warning => RuleResult::warning("body-full-stop", msg),
                RuleLevel::Disabled => RuleResult::pass("body-full-stop"),
            }
        }
    }

    fn validate_body_leading_blank(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_leading_blank;
        if !rule.is_active() {
            return RuleResult::pass("body-leading-blank");
        }

        // Check if there's a blank line between header and body
        let lines: Vec<&str> = commit.raw.lines().collect();

        if lines.len() < 2 {
            return RuleResult::pass("body-leading-blank");
        }

        // If there's no body, skip this check
        if commit.body.is_none() && commit.footer.is_none() {
            return RuleResult::pass("body-leading-blank");
        }

        let has_blank = lines.get(1).map(|l| l.trim().is_empty()).unwrap_or(false);

        let result = match rule.applicable {
            Applicable::Always => has_blank,
            Applicable::Never => !has_blank,
        };

        if result {
            RuleResult::pass("body-leading-blank")
        } else {
            let msg = "body must have leading blank line".to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-leading-blank", msg),
                RuleLevel::Warning => RuleResult::warning("body-leading-blank", msg),
                RuleLevel::Disabled => RuleResult::pass("body-leading-blank"),
            }
        }
    }

    fn validate_body_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_max_length;
        if !rule.is_active() {
            return RuleResult::pass("body-max-length");
        }

        let len = commit.body.as_ref().map(|b| b.chars().count()).unwrap_or(0);
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("body-max-length")
        } else {
            let msg = format!("body must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("body-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("body-max-length"),
            }
        }
    }

    fn validate_body_max_line_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_max_line_length;
        if !rule.is_active() {
            return RuleResult::pass("body-max-line-length");
        }

        let body = match &commit.body {
            Some(b) => b,
            None => return RuleResult::pass("body-max-line-length"),
        };

        let max = rule.value;
        let mut longest_line = 0;
        let mut line_num = 0;

        for (i, line) in body.lines().enumerate() {
            let len = line.chars().count();
            if len > longest_line {
                longest_line = len;
                line_num = i + 1;
            }
        }

        let passes = longest_line <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("body-max-line-length")
        } else {
            let msg = format!("body line {} must not be longer than {} characters (found {})", line_num, max, longest_line);
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-max-line-length", msg),
                RuleLevel::Warning => RuleResult::warning("body-max-line-length", msg),
                RuleLevel::Disabled => RuleResult::pass("body-max-line-length"),
            }
        }
    }

    fn validate_body_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.body_min_length;
        if !rule.is_active() {
            return RuleResult::pass("body-min-length");
        }

        let body = match &commit.body {
            Some(b) => b,
            None => return RuleResult::pass("body-min-length"),
        };

        let len = body.chars().count();
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("body-min-length")
        } else {
            let msg = format!("body must be at least {} characters", min);
            match rule.level {
                RuleLevel::Error => RuleResult::error("body-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("body-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("body-min-length"),
            }
        }
    }

    // ==================== Footer Rules ====================

    fn validate_footer_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.footer_empty;
        if !rule.is_active() {
            return RuleResult::pass("footer-empty");
        }

        let is_empty = commit.footer.as_ref().map(|f| f.trim().is_empty()).unwrap_or(true);

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("footer-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "footer must be empty",
                Applicable::Never => "footer may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("footer-empty", msg),
                RuleLevel::Warning => RuleResult::warning("footer-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("footer-empty"),
            }
        }
    }

    fn validate_footer_leading_blank(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.footer_leading_blank;
        if !rule.is_active() {
            return RuleResult::pass("footer-leading-blank");
        }

        // If there's no footer, skip
        if commit.footer.is_none() {
            return RuleResult::pass("footer-leading-blank");
        }

        // Check if there's a blank line before the footer
        // This is complex to check, so we approximate by checking if body ends with blank
        let lines: Vec<&str> = commit.raw.lines().collect();

        // Find where footer starts
        let footer_text = commit.footer.as_ref().unwrap();
        let footer_first_line = footer_text.lines().next().unwrap_or("");

        let footer_line_idx = lines.iter().position(|l| *l == footer_first_line);

        if let Some(idx) = footer_line_idx {
            if idx > 0 {
                let has_blank = lines.get(idx - 1).map(|l| l.trim().is_empty()).unwrap_or(false);

                let result = match rule.applicable {
                    Applicable::Always => has_blank,
                    Applicable::Never => !has_blank,
                };

                if !result {
                    let msg = "footer must have leading blank line".to_string();
                    return match rule.level {
                        RuleLevel::Error => RuleResult::error("footer-leading-blank", msg),
                        RuleLevel::Warning => RuleResult::warning("footer-leading-blank", msg),
                        RuleLevel::Disabled => RuleResult::pass("footer-leading-blank"),
                    };
                }
            }
        }

        RuleResult::pass("footer-leading-blank")
    }

    fn validate_footer_max_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.footer_max_length;
        if !rule.is_active() {
            return RuleResult::pass("footer-max-length");
        }

        let len = commit.footer.as_ref().map(|f| f.chars().count()).unwrap_or(0);
        let max = rule.value;
        let passes = len <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("footer-max-length")
        } else {
            let msg = format!("footer must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("footer-max-length", msg),
                RuleLevel::Warning => RuleResult::warning("footer-max-length", msg),
                RuleLevel::Disabled => RuleResult::pass("footer-max-length"),
            }
        }
    }

    fn validate_footer_max_line_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.footer_max_line_length;
        if !rule.is_active() {
            return RuleResult::pass("footer-max-line-length");
        }

        let footer = match &commit.footer {
            Some(f) => f,
            None => return RuleResult::pass("footer-max-line-length"),
        };

        let max = rule.value;
        let mut longest_line = 0;

        for line in footer.lines() {
            let len = line.chars().count();
            if len > longest_line {
                longest_line = len;
            }
        }

        let passes = longest_line <= max;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("footer-max-line-length")
        } else {
            let msg = format!("footer line must not be longer than {} characters", max);
            match rule.level {
                RuleLevel::Error => RuleResult::error("footer-max-line-length", msg),
                RuleLevel::Warning => RuleResult::warning("footer-max-line-length", msg),
                RuleLevel::Disabled => RuleResult::pass("footer-max-line-length"),
            }
        }
    }

    fn validate_footer_min_length(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.footer_min_length;
        if !rule.is_active() {
            return RuleResult::pass("footer-min-length");
        }

        let footer = match &commit.footer {
            Some(f) => f,
            None => return RuleResult::pass("footer-min-length"),
        };

        let len = footer.chars().count();
        let min = rule.value;
        let passes = len >= min;

        let result = match rule.applicable {
            Applicable::Always => passes,
            Applicable::Never => !passes,
        };

        if result {
            RuleResult::pass("footer-min-length")
        } else {
            let msg = format!("footer must be at least {} characters", min);
            match rule.level {
                RuleLevel::Error => RuleResult::error("footer-min-length", msg),
                RuleLevel::Warning => RuleResult::warning("footer-min-length", msg),
                RuleLevel::Disabled => RuleResult::pass("footer-min-length"),
            }
        }
    }

    // ==================== Other Rules ====================

    fn validate_references_empty(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.references_empty;
        if !rule.is_active() {
            return RuleResult::pass("references-empty");
        }

        let is_empty = commit.references.is_empty();

        let result = match rule.applicable {
            Applicable::Always => is_empty,
            Applicable::Never => !is_empty,
        };

        if result {
            RuleResult::pass("references-empty")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => "references must be empty",
                Applicable::Never => "references may not be empty",
            }.to_string();
            match rule.level {
                RuleLevel::Error => RuleResult::error("references-empty", msg),
                RuleLevel::Warning => RuleResult::warning("references-empty", msg),
                RuleLevel::Disabled => RuleResult::pass("references-empty"),
            }
        }
    }

    fn validate_signed_off_by(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.signed_off_by;
        if !rule.is_active() {
            return RuleResult::pass("signed-off-by");
        }

        let pattern = if rule.value.is_empty() {
            "Signed-off-by:"
        } else {
            &rule.value
        };

        let has_signoff = commit.trailers.keys().any(|k| k.starts_with("Signed-off-by"));

        let result = match rule.applicable {
            Applicable::Always => has_signoff,
            Applicable::Never => !has_signoff,
        };

        if result {
            RuleResult::pass("signed-off-by")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => format!("message must be signed off (contain '{}')", pattern),
                Applicable::Never => format!("message must not be signed off (contain '{}')", pattern),
            };
            match rule.level {
                RuleLevel::Error => RuleResult::error("signed-off-by", msg),
                RuleLevel::Warning => RuleResult::warning("signed-off-by", msg),
                RuleLevel::Disabled => RuleResult::pass("signed-off-by"),
            }
        }
    }

    fn validate_trailer_exists(&self, commit: &ConventionalCommit) -> RuleResult {
        let rule = &self.config.rules.trailer_exists;
        if !rule.is_active() || rule.value.is_empty() {
            return RuleResult::pass("trailer-exists");
        }

        let trailer = &rule.value;
        let has_trailer = commit.trailers.keys().any(|k| k == trailer);

        let result = match rule.applicable {
            Applicable::Always => has_trailer,
            Applicable::Never => !has_trailer,
        };

        if result {
            RuleResult::pass("trailer-exists")
        } else {
            let msg = match rule.applicable {
                Applicable::Always => format!("message must have trailer '{}'", trailer),
                Applicable::Never => format!("message must not have trailer '{}'", trailer),
            };
            match rule.level {
                RuleLevel::Error => RuleResult::error("trailer-exists", msg),
                RuleLevel::Warning => RuleResult::warning("trailer-exists", msg),
                RuleLevel::Disabled => RuleResult::pass("trailer-exists"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_valid_commit() {
        let config = default_config();
        let validator = Validator::new(config);
        let result = validator.validate("feat: add new feature");
        assert!(result.valid, "Expected valid but got errors: {:?}", result.failures());
    }

    #[test]
    fn test_invalid_type() {
        let config = default_config();
        let validator = Validator::new(config);
        let result = validator.validate("invalid: this is not a valid type");
        assert!(!result.valid);
        assert!(result.results.iter().any(|r| r.name == "type-enum" && !r.valid));
    }

    #[test]
    fn test_empty_subject() {
        let config = default_config();
        let validator = Validator::new(config);
        let result = validator.validate("feat: ");
        assert!(!result.valid);
        assert!(result.results.iter().any(|r| r.name == "subject-empty" && !r.valid));
    }

    #[test]
    fn test_subject_with_full_stop() {
        let config = default_config();
        let validator = Validator::new(config);
        let result = validator.validate("feat: add new feature.");
        assert!(!result.valid);
        assert!(result.results.iter().any(|r| r.name == "subject-full-stop" && !r.valid));
    }

    #[test]
    fn test_header_too_long() {
        let config = default_config();
        let validator = Validator::new(config);
        let long_subject = "a".repeat(150);
        let result = validator.validate(&format!("feat: {}", long_subject));
        assert!(!result.valid);
        assert!(result.results.iter().any(|r| r.name == "header-max-length" && !r.valid));
    }
}
