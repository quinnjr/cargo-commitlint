use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_rules")]
    pub rules: Rules,
    #[serde(default)]
    pub parser: Parser,
    #[serde(default)]
    pub ignores: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    #[serde(default = "default_type_rule")]
    pub r#type: TypeRule,
    #[serde(default = "default_scope_rule")]
    pub scope: ScopeRule,
    #[serde(default = "default_subject_case")]
    pub subject_case: Vec<String>,
    #[serde(default = "default_subject_empty")]
    pub subject_empty: bool,
    #[serde(default = "default_subject_full_stop")]
    pub subject_full_stop: String,
    #[serde(default = "default_header_max_length")]
    pub header_max_length: usize,
    #[serde(default = "default_header_min_length")]
    pub header_min_length: usize,
    #[serde(default = "default_body_leading_blank")]
    pub body_leading_blank: bool,
    #[serde(default = "default_body_max_line_length")]
    pub body_max_line_length: usize,
    #[serde(default = "default_footer_leading_blank")]
    pub footer_leading_blank: bool,
    #[serde(default = "default_footer_max_line_length")]
    pub footer_max_line_length: usize,
    #[serde(default = "default_allow_breaking")]
    pub allow_breaking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRule {
    #[serde(default = "default_type_enum")]
    pub r#enum: Vec<String>,
    #[serde(default = "default_type_case")]
    pub case: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeRule {
    #[serde(default = "default_scope_enum")]
    pub r#enum: Vec<String>,
    #[serde(default = "default_scope_case")]
    pub case: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parser {
    #[serde(default = "default_parser_pattern")]
    pub pattern: String,
    #[serde(default = "default_parser_correspondence")]
    pub correspondence: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: default_rules(),
            parser: Parser::default(),
            ignores: Vec::new(),
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            pattern: default_parser_pattern(),
            correspondence: default_parser_correspondence(),
        }
    }
}

fn default_rules() -> Rules {
    Rules {
        r#type: default_type_rule(),
        scope: default_scope_rule(),
        subject_case: default_subject_case(),
        subject_empty: default_subject_empty(),
        subject_full_stop: default_subject_full_stop(),
        header_max_length: default_header_max_length(),
        header_min_length: default_header_min_length(),
        body_leading_blank: default_body_leading_blank(),
        body_max_line_length: default_body_max_line_length(),
        footer_leading_blank: default_footer_leading_blank(),
        footer_max_line_length: default_footer_max_line_length(),
        allow_breaking: default_allow_breaking(),
    }
}

fn default_type_rule() -> TypeRule {
    TypeRule {
        r#enum: default_type_enum(),
        case: default_type_case(),
    }
}

fn default_scope_rule() -> ScopeRule {
    ScopeRule {
        r#enum: default_scope_enum(),
        case: default_scope_case(),
    }
}

fn default_type_enum() -> Vec<String> {
    vec![
        "build".to_string(),
        "chore".to_string(),
        "ci".to_string(),
        "docs".to_string(),
        "feat".to_string(),
        "fix".to_string(),
        "perf".to_string(),
        "refactor".to_string(),
        "revert".to_string(),
        "style".to_string(),
        "test".to_string(),
    ]
}

fn default_type_case() -> String {
    "lowercase".to_string()
}

fn default_scope_enum() -> Vec<String> {
    Vec::new()
}

fn default_scope_case() -> String {
    "lowercase".to_string()
}

fn default_subject_case() -> Vec<String> {
    vec!["sentence-case".to_string()]
}

fn default_subject_empty() -> bool {
    false
}

fn default_subject_full_stop() -> String {
    ".".to_string()
}

fn default_header_max_length() -> usize {
    72
}

fn default_header_min_length() -> usize {
    0
}

fn default_body_leading_blank() -> bool {
    true
}

fn default_body_max_line_length() -> usize {
    100
}

fn default_footer_leading_blank() -> bool {
    true
}

fn default_footer_max_line_length() -> usize {
    100
}

fn default_allow_breaking() -> bool {
    true
}

pub(crate) const DEFAULT_PARSER_PATTERN: &str =
    r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s(?P<subject>.*)$";

fn default_parser_pattern() -> String {
    DEFAULT_PARSER_PATTERN.to_string()
}

fn default_parser_correspondence() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("type".to_string(), "type".to_string());
    map.insert("scope".to_string(), "scope".to_string());
    map.insert("subject".to_string(), "subject".to_string());
    map.insert("breaking".to_string(), "breaking".to_string());
    map
}

impl Config {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        if !path.exists() {
            anyhow::bail!("config file not found: {}", path.display());
        }
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_default_locations() -> anyhow::Result<Self> {
        Self::from_locations(&std::env::current_dir()?)
    }

    /// Probes the standard config locations relative to `base`, returning the
    /// first match. Falls back to [`Config::default`] when none are present.
    pub fn from_locations(base: &std::path::Path) -> anyhow::Result<Self> {
        let config_path = base.join("commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        let config_path = base.join(".commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        let config_path = base.join(".cargo").join("commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        Ok(Config::default())
    }

    /// Verify that every user-supplied regex in the config actually compiles.
    ///
    /// Called once after loading so a malformed pattern is reported as the
    /// configuration error it is, rather than silently degrading validation on
    /// every commit.
    ///
    /// This is the fail-fast front door: callers that load a config should run
    /// it before handing the config to [`crate::validator::Validator`], which
    /// keeps a defensive warn-and-drop fallback for directly constructed
    /// configs that never pass through here.
    pub fn validate(&self) -> anyhow::Result<()> {
        let mut errors = Vec::new();

        for pattern in &self.ignores {
            if let Err(e) = Regex::new(pattern) {
                errors.push(format!("invalid ignore pattern '{}': {}", pattern, e));
            }
        }

        // The default pattern is known-good, so skip the needless compile.
        if self.parser.pattern != DEFAULT_PARSER_PATTERN {
            if let Err(e) = Regex::new(&self.parser.pattern) {
                errors.push(format!(
                    "invalid parser.pattern '{}': {}",
                    self.parser.pattern, e
                ));
            }
        }

        if !errors.is_empty() {
            anyhow::bail!(errors.join("\n"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_errors_on_nonexistent_path() {
        let path = std::path::Path::new("/nonexistent/path/to/commitlint.toml");
        let result = Config::from_file(path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("config file not found"));
    }

    #[test]
    fn test_example_config_parses_with_keys_in_correct_tables() {
        // Embedded at compile time so the assertions run regardless of the
        // working directory the test harness happens to use.
        let content = include_str!("../commitlint.example.toml");
        let config: Config = toml::from_str(content).expect("example config must parse");
        assert_eq!(config.rules.subject_case, vec!["sentence-case"]);
        assert_eq!(config.rules.header_max_length, 72);
        assert!(config.rules.allow_breaking);
        assert_eq!(config.rules.r#type.r#enum.len(), 11);
        assert_eq!(config.parser.correspondence["type"], "type");
        assert!(config.ignores.is_empty());
    }

    /// Creates (and returns) a unique, empty scratch directory for one test.
    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cargo-commitlint-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    fn write_config(path: &std::path::Path, header_max_length: usize) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create config parent dir");
        }
        std::fs::write(
            path,
            format!("[rules]\nheader_max_length = {header_max_length}\n"),
        )
        .expect("write config fixture");
    }

    #[test]
    fn test_from_locations_prefers_commitlint_toml() {
        let dir = scratch_dir("prefers-plain");
        write_config(&dir.join("commitlint.toml"), 50);
        write_config(&dir.join(".commitlint.toml"), 60);

        let config = Config::from_locations(&dir).expect("config must load");
        assert_eq!(config.rules.header_max_length, 50);

        std::fs::remove_dir_all(&dir).expect("clean up scratch dir");
    }

    #[test]
    fn test_from_locations_falls_back_to_dot_prefixed() {
        let dir = scratch_dir("dot-prefixed");
        write_config(&dir.join(".commitlint.toml"), 60);

        let config = Config::from_locations(&dir).expect("config must load");
        assert_eq!(config.rules.header_max_length, 60);

        std::fs::remove_dir_all(&dir).expect("clean up scratch dir");
    }

    #[test]
    fn test_from_locations_finds_cargo_subdir() {
        let dir = scratch_dir("cargo-subdir");
        write_config(&dir.join(".cargo").join("commitlint.toml"), 80);

        let config = Config::from_locations(&dir).expect("config must load");
        assert_eq!(config.rules.header_max_length, 80);

        std::fs::remove_dir_all(&dir).expect("clean up scratch dir");
    }

    #[test]
    fn test_from_locations_empty_dir_returns_defaults() {
        let dir = scratch_dir("empty");

        let config = Config::from_locations(&dir).expect("config must load");
        assert_eq!(config.rules.header_max_length, 72);
        assert!(config.rules.allow_breaking);

        std::fs::remove_dir_all(&dir).expect("clean up scratch dir");
    }

    #[test]
    fn test_validate_accepts_default_config() {
        assert!(Config::default().validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_invalid_ignore_pattern() {
        let config = Config {
            ignores: vec!["[".to_string()],
            ..Config::default()
        };

        let result = config.validate();
        assert!(result.is_err());
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("invalid ignore pattern '['"),
            "message must name the offending pattern, got: {message}"
        );
    }

    #[test]
    fn test_validate_rejects_invalid_parser_pattern() {
        let mut config = Config::default();
        config.parser.pattern = "(?P<type".to_string();

        let result = config.validate();
        assert!(result.is_err());
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("invalid parser.pattern '(?P<type'"),
            "message must name the offending pattern, got: {message}"
        );
    }

    #[test]
    fn test_validate_reports_all_invalid_patterns_together() {
        let config = Config {
            ignores: vec!["[".to_string(), "(?P<unclosed".to_string()],
            ..Config::default()
        };

        let result = config.validate();
        assert!(result.is_err());
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("invalid ignore pattern '['"),
            "first bad pattern must be reported, got: {message}"
        );
        assert!(
            message.contains("invalid ignore pattern '(?P<unclosed'"),
            "second bad pattern must be reported, got: {message}"
        );
    }

    #[test]
    fn test_validate_accepts_valid_custom_parser_pattern() {
        let mut config = Config::default();
        config.parser.pattern = r"^(?P<type>\w+):\s(?P<subject>.+)$".to_string();
        config.ignores = vec!["^Merge".to_string(), r"^Revert\s".to_string()];

        assert!(config.validate().is_ok());
    }
}
