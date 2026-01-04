//! Configuration module supporting multiple formats
//!
//! Supports:
//! - commitlint.toml / .commitlint.toml (TOML format - Rust native)
//! - .commitlintrc (JSON/YAML auto-detect)
//! - .commitlintrc.json (JSON format)
//! - .commitlintrc.yaml / .commitlintrc.yml (YAML format)
//! - package.json "commitlint" field

use crate::rules::Rules;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Extends shared configuration
    #[serde(default)]
    pub extends: Vec<String>,

    /// Parser preset name
    #[serde(default, rename = "parserPreset")]
    pub parser_preset: Option<String>,

    /// Parser configuration
    #[serde(default)]
    pub parser: ParserConfig,

    /// All rules
    #[serde(default)]
    pub rules: Rules,

    /// Patterns to ignore (commits matching these regex patterns skip validation)
    #[serde(default)]
    pub ignores: Vec<String>,

    /// Default ignores (merge commits, etc.)
    #[serde(default = "default_true")]
    #[serde(rename = "defaultIgnores")]
    pub default_ignores: bool,

    /// Help URL to display in error messages
    #[serde(default, rename = "helpUrl")]
    pub help_url: Option<String>,

    /// Prompt configuration (for interactive mode)
    #[serde(default)]
    pub prompt: PromptConfig,
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            extends: Vec::new(),
            parser_preset: None,
            parser: ParserConfig::default(),
            rules: Rules::default(),
            ignores: Vec::new(),
            default_ignores: true,
            help_url: None,
            prompt: PromptConfig::default(),
        }
    }
}

/// Parser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParserConfig {
    /// Regex pattern for parsing commit header
    #[serde(default = "default_header_pattern")]
    pub header_pattern: String,

    /// Groups correspondence for header pattern
    #[serde(default = "default_header_correspondence")]
    pub header_correspondence: Vec<String>,

    /// Note keywords (e.g., "BREAKING CHANGE")
    #[serde(default = "default_note_keywords")]
    pub note_keywords: Vec<String>,

    /// Reference actions (e.g., "close", "closes", "fix", "fixes")
    #[serde(default = "default_reference_actions")]
    pub reference_actions: Vec<String>,

    /// Whether to merge the parser preset with configuration
    #[serde(default)]
    pub merge_preset: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            header_pattern: default_header_pattern(),
            header_correspondence: default_header_correspondence(),
            note_keywords: default_note_keywords(),
            reference_actions: default_reference_actions(),
            merge_preset: false,
        }
    }
}

fn default_header_pattern() -> String {
    r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s*(?P<subject>.*)$".to_string()
}

fn default_header_correspondence() -> Vec<String> {
    vec![
        "type".to_string(),
        "scope".to_string(),
        "subject".to_string(),
    ]
}

fn default_note_keywords() -> Vec<String> {
    vec!["BREAKING CHANGE".to_string(), "BREAKING-CHANGE".to_string()]
}

fn default_reference_actions() -> Vec<String> {
    vec![
        "close".to_string(),
        "closes".to_string(),
        "closed".to_string(),
        "fix".to_string(),
        "fixes".to_string(),
        "fixed".to_string(),
        "resolve".to_string(),
        "resolves".to_string(),
        "resolved".to_string(),
    ]
}

/// Prompt configuration for interactive commit message creation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PromptConfig {
    pub messages: PromptMessages,
    pub questions: PromptQuestions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PromptMessages {
    pub skip: String,
    pub max: String,
    pub min: String,
    #[serde(rename = "emptyWarning")]
    pub empty_warning: String,
    #[serde(rename = "upperLimitWarning")]
    pub upper_limit_warning: String,
    #[serde(rename = "lowerLimitWarning")]
    pub lower_limit_warning: String,
}

impl Default for PromptMessages {
    fn default() -> Self {
        Self {
            skip: "(press enter to skip)".to_string(),
            max: "upper %d chars".to_string(),
            min: "%d chars at least".to_string(),
            empty_warning: "can not be empty".to_string(),
            upper_limit_warning: "over limit".to_string(),
            lower_limit_warning: "below limit".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PromptQuestions {
    #[serde(rename = "type")]
    pub commit_type: QuestionConfig,
    pub scope: QuestionConfig,
    pub subject: QuestionConfig,
    pub body: QuestionConfig,
    #[serde(rename = "isBreaking")]
    pub is_breaking: QuestionConfig,
    #[serde(rename = "breakingBody")]
    pub breaking_body: QuestionConfig,
    #[serde(rename = "breaking")]
    pub breaking: QuestionConfig,
    #[serde(rename = "isIssueAffected")]
    pub is_issue_affected: QuestionConfig,
    #[serde(rename = "issuesBody")]
    pub issues_body: QuestionConfig,
    pub issues: QuestionConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct QuestionConfig {
    pub description: Option<String>,
    pub messages: Option<serde_json::Value>,
    #[serde(rename = "enum")]
    pub enum_values: Option<serde_json::Value>,
}

/// Configuration file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Toml,
    Json,
    Yaml,
}

impl Config {
    /// Load configuration from a specific file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let format = Self::detect_format(path, &content);

        let mut config: Config = match format {
            ConfigFormat::Toml => toml::from_str(&content)
                .with_context(|| format!("Failed to parse TOML config: {}", path.display()))?,
            ConfigFormat::Json => serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse JSON config: {}", path.display()))?,
            ConfigFormat::Yaml => serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse YAML config: {}", path.display()))?,
        };

        // Process extends
        if !config.extends.is_empty() {
            config = config.apply_extends(path.parent())?;
        }

        Ok(config)
    }

    /// Detect configuration format from file extension and content
    fn detect_format(path: &Path, content: &str) -> ConfigFormat {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext {
            "toml" => ConfigFormat::Toml,
            "json" => ConfigFormat::Json,
            "yaml" | "yml" => ConfigFormat::Yaml,
            _ => {
                // Try to auto-detect from content
                let trimmed = content.trim();
                if trimmed.starts_with('{') {
                    ConfigFormat::Json
                } else if trimmed.contains(": ") || trimmed.starts_with("---") || trimmed.contains(":\n") {
                    ConfigFormat::Yaml
                } else {
                    ConfigFormat::Toml
                }
            }
        }
    }

    /// Apply extends configuration
    fn apply_extends(mut self, base_dir: Option<&Path>) -> Result<Self> {
        for extend in self.extends.clone() {
            let preset_rules = Self::load_preset(&extend, base_dir)?;
            self.rules = Self::merge_rules(preset_rules, self.rules);
        }
        self.extends.clear();
        Ok(self)
    }

    /// Load a preset configuration
    fn load_preset(name: &str, _base_dir: Option<&Path>) -> Result<Rules> {
        // Built-in presets
        match name {
            "@commitlint/config-conventional" | "conventional" => {
                Ok(Rules::conventional())
            }
            "@commitlint/config-angular" | "angular" => {
                // Angular preset is similar to conventional
                Ok(Rules::conventional())
            }
            _ => {
                // Try to load as a file path
                anyhow::bail!("Unknown preset: {}. Use 'conventional' or '@commitlint/config-conventional'", name)
            }
        }
    }

    /// Merge two rulesets (override takes precedence)
    fn merge_rules(_base: Rules, override_rules: Rules) -> Rules {
        // For simplicity, override_rules takes full precedence
        // A more sophisticated merge would check which rules are explicitly set
        override_rules.clone()
    }

    /// Load configuration from default locations
    pub fn from_default_locations() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::from_directory(&cwd)
    }

    /// Load configuration from a directory (searching up the tree)
    pub fn from_directory(dir: &Path) -> Result<Self> {
        let config_files = [
            "commitlint.toml",
            ".commitlint.toml",
            ".commitlintrc",
            ".commitlintrc.json",
            ".commitlintrc.yaml",
            ".commitlintrc.yml",
            ".commitlintrc.toml",
            "commitlint.config.json",
        ];

        let mut current = Some(dir);

        while let Some(dir) = current {
            // Check for config files
            for filename in &config_files {
                let path = dir.join(filename);
                if path.exists() {
                    return Self::from_file(&path);
                }
            }

            // Check package.json
            let package_json = dir.join("package.json");
            if package_json.exists() {
                if let Ok(config) = Self::from_package_json(&package_json) {
                    return Ok(config);
                }
            }

            // Check .cargo/commitlint.toml
            let cargo_config = dir.join(".cargo").join("commitlint.toml");
            if cargo_config.exists() {
                return Self::from_file(&cargo_config);
            }

            current = dir.parent();
        }

        // No config found, use default
        Ok(Config::default())
    }

    /// Load configuration from package.json "commitlint" field
    fn from_package_json(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let package: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(commitlint) = package.get("commitlint") {
            let config: Config = serde_json::from_value(commitlint.clone())?;
            return Ok(config);
        }

        anyhow::bail!("No 'commitlint' field in package.json")
    }

    /// Get the path where config was found, or None if using defaults
    pub fn find_config_path(dir: &Path) -> Option<PathBuf> {
        let config_files = [
            "commitlint.toml",
            ".commitlint.toml",
            ".commitlintrc",
            ".commitlintrc.json",
            ".commitlintrc.yaml",
            ".commitlintrc.yml",
            ".commitlintrc.toml",
            "commitlint.config.json",
        ];

        let mut current = Some(dir);

        while let Some(dir) = current {
            for filename in &config_files {
                let path = dir.join(filename);
                if path.exists() {
                    return Some(path);
                }
            }

            let package_json = dir.join("package.json");
            if package_json.exists() {
                if let Ok(content) = std::fs::read_to_string(&package_json) {
                    if let Ok(package) = serde_json::from_str::<serde_json::Value>(&content) {
                        if package.get("commitlint").is_some() {
                            return Some(package_json);
                        }
                    }
                }
            }

            let cargo_config = dir.join(".cargo").join("commitlint.toml");
            if cargo_config.exists() {
                return Some(cargo_config);
            }

            current = dir.parent();
        }

        None
    }

    /// Check if a commit message should be ignored
    pub fn should_ignore(&self, message: &str) -> bool {
        // Default ignores
        if self.default_ignores {
            // Skip merge commits
            if message.starts_with("Merge ") {
                return true;
            }
            // Skip revert commits (handled differently)
            if message.starts_with("Revert ") {
                return true;
            }
            // Skip initial commit
            if message.trim() == "Initial commit" || message.trim() == "initial commit" {
                return true;
            }
            // Skip WIP commits
            if message.starts_with("WIP") || message.starts_with("wip") || message.starts_with("fixup!") || message.starts_with("squash!") {
                return true;
            }
        }

        // Custom ignores
        for pattern in &self.ignores {
            if let Ok(re) = regex::Regex::new(pattern) {
                if re.is_match(message) {
                    return true;
                }
            }
        }

        false
    }

    /// Serialize config to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize config to JSON")
    }

    /// Serialize config to TOML
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).context("Failed to serialize config to TOML")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.default_ignores);
        assert!(config.extends.is_empty());
    }

    #[test]
    fn test_should_ignore_merge() {
        let config = Config::default();
        assert!(config.should_ignore("Merge branch 'main' into feature"));
        assert!(config.should_ignore("Merge pull request #123"));
    }

    #[test]
    fn test_should_not_ignore_regular() {
        let config = Config::default();
        assert!(!config.should_ignore("feat: add new feature"));
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(
            Config::detect_format(Path::new("test.toml"), ""),
            ConfigFormat::Toml
        );
        assert_eq!(
            Config::detect_format(Path::new("test.json"), ""),
            ConfigFormat::Json
        );
        assert_eq!(
            Config::detect_format(Path::new("test.yaml"), ""),
            ConfigFormat::Yaml
        );
        assert_eq!(
            Config::detect_format(Path::new(".commitlintrc"), "{}"),
            ConfigFormat::Json
        );
        assert_eq!(
            Config::detect_format(Path::new(".commitlintrc"), "rules:\n  type-enum:"),
            ConfigFormat::Yaml
        );
    }
}
