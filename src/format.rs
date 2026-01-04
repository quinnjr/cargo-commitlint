//! Output formatting module
//!
//! Provides different output formats: text (default), json, commitlint-style

use crate::validator::ValidationResult;
use colored::Colorize;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Compact,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" | "default" | "" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "compact" => Ok(OutputFormat::Compact),
            _ => Err(format!("Unknown format: {}. Use 'text', 'json', or 'compact'", s)),
        }
    }
}

/// Format validation results for output
pub struct Formatter {
    format: OutputFormat,
    color: bool,
    verbose: bool,
    help_url: Option<String>,
}

impl Formatter {
    pub fn new(format: OutputFormat, color: bool, verbose: bool, help_url: Option<String>) -> Self {
        Self {
            format,
            color,
            verbose,
            help_url,
        }
    }
    
    /// Format a single validation result
    pub fn format_result(&self, result: &ValidationResult) -> String {
        match self.format {
            OutputFormat::Text => self.format_text(result),
            OutputFormat::Json => self.format_json(result),
            OutputFormat::Compact => self.format_compact(result),
        }
    }
    
    /// Format multiple validation results
    pub fn format_results(&self, results: &[ValidationResult]) -> String {
        match self.format {
            OutputFormat::Json => self.format_json_array(results),
            _ => {
                results
                    .iter()
                    .map(|r| self.format_result(r))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
    
    fn format_text(&self, result: &ValidationResult) -> String {
        let mut output = String::new();
        
        // Input header
        let header = if result.commit.header.len() > 72 {
            format!("{}...", &result.commit.header[..69])
        } else {
            result.commit.header.clone()
        };
        
        if self.color {
            output.push_str(&format!("⧗   input: {}\n", header.dimmed()));
        } else {
            output.push_str(&format!("⧗   input: {}\n", header));
        }
        
        // If valid and not verbose, just show success
        if result.valid && !self.verbose {
            if self.color {
                output.push_str(&format!("{}  found 0 problems, 0 warnings\n", "✔".green()));
            } else {
                output.push_str("✔  found 0 problems, 0 warnings\n");
            }
            return output;
        }
        
        // Show problems
        for rule_result in &result.results {
            if rule_result.valid {
                if self.verbose {
                    if self.color {
                        output.push_str(&format!(
                            "{}   {} [{}]\n",
                            "✔".green(),
                            rule_result.name.dimmed(),
                            "passed".dimmed()
                        ));
                    } else {
                        output.push_str(&format!("✔   {} [passed]\n", rule_result.name));
                    }
                }
                continue;
            }
            
            let (symbol, level_str) = match rule_result.level {
                crate::rules::RuleLevel::Error => {
                    if self.color {
                        ("✖".red().to_string(), "error")
                    } else {
                        ("✖".to_string(), "error")
                    }
                }
                crate::rules::RuleLevel::Warning => {
                    if self.color {
                        ("⚠".yellow().to_string(), "warning")
                    } else {
                        ("⚠".to_string(), "warning")
                    }
                }
                crate::rules::RuleLevel::Disabled => continue,
            };
            
            if self.color {
                output.push_str(&format!(
                    "{}   {} {} [{}]\n",
                    symbol,
                    rule_result.message,
                    format!("[{}]", level_str).dimmed(),
                    rule_result.name.cyan()
                ));
            } else {
                output.push_str(&format!(
                    "{}   {} [{}] [{}]\n",
                    symbol,
                    rule_result.message,
                    level_str,
                    rule_result.name
                ));
            }
        }
        
        // Summary
        if self.color {
            let status = if result.errors > 0 {
                "✖".red()
            } else if result.warnings > 0 {
                "⚠".yellow()
            } else {
                "✔".green()
            };
            output.push_str(&format!(
                "\n{}  found {} problems, {} warnings\n",
                status,
                if result.errors > 0 { result.errors.to_string().red().to_string() } else { "0".to_string() },
                if result.warnings > 0 { result.warnings.to_string().yellow().to_string() } else { "0".to_string() }
            ));
        } else {
            let status = if result.errors > 0 { "✖" } else if result.warnings > 0 { "⚠" } else { "✔" };
            output.push_str(&format!(
                "\n{}  found {} problems, {} warnings\n",
                status, result.errors, result.warnings
            ));
        }
        
        // Help URL
        if !result.valid {
            if let Some(ref url) = self.help_url {
                if self.color {
                    output.push_str(&format!("\n{}  {}\n", "ℹ".blue(), url.underline()));
                } else {
                    output.push_str(&format!("\nℹ  {}\n", url));
                }
            }
        }
        
        output
    }
    
    fn format_json(&self, result: &ValidationResult) -> String {
        let errors: Vec<serde_json::Value> = result
            .results
            .iter()
            .filter(|r| !r.valid && r.level == crate::rules::RuleLevel::Error)
            .map(|r| {
                serde_json::json!({
                    "name": r.name,
                    "message": r.message,
                    "level": 2
                })
            })
            .collect();
        
        let warnings: Vec<serde_json::Value> = result
            .results
            .iter()
            .filter(|r| !r.valid && r.level == crate::rules::RuleLevel::Warning)
            .map(|r| {
                serde_json::json!({
                    "name": r.name,
                    "message": r.message,
                    "level": 1
                })
            })
            .collect();
        
        serde_json::json!({
            "valid": result.valid,
            "errorCount": result.errors,
            "warningCount": result.warnings,
            "input": result.input,
            "errors": errors,
            "warnings": warnings,
        })
        .to_string()
    }
    
    fn format_json_array(&self, results: &[ValidationResult]) -> String {
        let items: Vec<serde_json::Value> = results
            .iter()
            .map(|r| serde_json::from_str(&self.format_json(r)).unwrap())
            .collect();
        
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }
    
    fn format_compact(&self, result: &ValidationResult) -> String {
        let mut output = String::new();
        
        if result.valid {
            output.push_str("✔ valid\n");
        } else {
            for rule_result in &result.results {
                if rule_result.valid {
                    continue;
                }
                
                let symbol = match rule_result.level {
                    crate::rules::RuleLevel::Error => "✖",
                    crate::rules::RuleLevel::Warning => "⚠",
                    crate::rules::RuleLevel::Disabled => continue,
                };
                
                output.push_str(&format!("{} {}: {}\n", symbol, rule_result.name, rule_result.message));
            }
        }
        
        output
    }
}

/// Print a summary for multiple commits
pub fn print_summary(results: &[ValidationResult], color: bool) {
    let total = results.len();
    let valid = results.iter().filter(|r| r.valid).count();
    let invalid = total - valid;
    let total_errors: usize = results.iter().map(|r| r.errors).sum();
    let total_warnings: usize = results.iter().map(|r| r.warnings).sum();
    
    println!();
    if color {
        if invalid > 0 {
            println!(
                "{}  {} commit(s) valid, {} invalid",
                "✖".red(),
                valid.to_string().green(),
                invalid.to_string().red()
            );
        } else {
            println!(
                "{}  {} commit(s) valid",
                "✔".green(),
                valid.to_string().green()
            );
        }
        println!(
            "   {} problems, {} warnings in total",
            if total_errors > 0 { total_errors.to_string().red().to_string() } else { "0".to_string() },
            if total_warnings > 0 { total_warnings.to_string().yellow().to_string() } else { "0".to_string() }
        );
    } else {
        if invalid > 0 {
            println!("✖  {} commit(s) valid, {} invalid", valid, invalid);
        } else {
            println!("✔  {} commit(s) valid", valid);
        }
        println!("   {} problems, {} warnings in total", total_errors, total_warnings);
    }
}
