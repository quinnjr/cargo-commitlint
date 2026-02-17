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

fn default_parser_pattern() -> String {
    r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s(?P<subject>.*)$".to_string()
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
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn from_default_locations() -> anyhow::Result<Self> {
        // Try to find config in common locations
        let current_dir = std::env::current_dir()?;

        // Check for commitlint.toml in current directory
        let config_path = current_dir.join("commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        // Check for .commitlint.toml in current directory
        let config_path = current_dir.join(".commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        // Check for commitlint.toml in .cargo directory
        let config_path = current_dir.join(".cargo").join("commitlint.toml");
        if config_path.exists() {
            return Self::from_file(&config_path);
        }

        // Default config
        Ok(Config::default())
    }
}
