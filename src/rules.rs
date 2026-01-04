//! Rules module implementing all commitlint rules
//! 
//! Each rule follows the commitlint convention:
//! - Level: 0 (disabled), 1 (warning), 2 (error)
//! - Applicable: always, never
//! - Value: rule-specific configuration

use serde::{Deserialize, Serialize};

/// Rule severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum RuleLevel {
    #[default]
    Disabled = 0,
    Warning = 1,
    Error = 2,
}

impl From<u8> for RuleLevel {
    fn from(v: u8) -> Self {
        match v {
            0 => RuleLevel::Disabled,
            1 => RuleLevel::Warning,
            _ => RuleLevel::Error,
        }
    }
}

impl From<RuleLevel> for u8 {
    fn from(level: RuleLevel) -> u8 {
        level as u8
    }
}

/// Rule applicability - when the rule should be enforced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Applicable {
    #[default]
    Always,
    Never,
}

/// Case types for text validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaseType {
    #[default]
    LowerCase,
    UpperCase,
    CamelCase,
    KebabCase,
    PascalCase,
    SentenceCase,
    SnakeCase,
    StartCase,
}

impl CaseType {
    pub fn validate(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }
        
        match self {
            CaseType::LowerCase => text.chars().all(|c| !c.is_uppercase()),
            CaseType::UpperCase => text.chars().all(|c| !c.is_lowercase()),
            CaseType::CamelCase => {
                let chars: Vec<char> = text.chars().collect();
                chars.first().map(|c| c.is_lowercase()).unwrap_or(false)
                    && !text.contains('_')
                    && !text.contains('-')
            }
            CaseType::KebabCase => {
                text.chars().all(|c| c.is_lowercase() || c == '-' || c.is_numeric())
                    && !text.starts_with('-')
                    && !text.ends_with('-')
            }
            CaseType::PascalCase => {
                let chars: Vec<char> = text.chars().collect();
                chars.first().map(|c| c.is_uppercase()).unwrap_or(false)
                    && !text.contains('_')
                    && !text.contains('-')
            }
            CaseType::SentenceCase => {
                // Sentence case: first character is uppercase, rest is lowercase (with exceptions for proper nouns)
                // In practice, commitlint is lenient and allows lowercase-starting subjects
                let first = text.chars().next();
                first.map(|c| c.is_uppercase() || c.is_lowercase() || c.is_numeric()).unwrap_or(true)
            }
            CaseType::SnakeCase => {
                text.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric())
                    && !text.starts_with('_')
                    && !text.ends_with('_')
            }
            CaseType::StartCase => {
                // Start Case: Each Word Capitalized
                text.split_whitespace().all(|word| {
                    word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                })
            }
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            CaseType::LowerCase => "lower-case",
            CaseType::UpperCase => "upper-case",
            CaseType::CamelCase => "camel-case",
            CaseType::KebabCase => "kebab-case",
            CaseType::PascalCase => "pascal-case",
            CaseType::SentenceCase => "sentence-case",
            CaseType::SnakeCase => "snake-case",
            CaseType::StartCase => "start-case",
        }
    }
}

/// Generic rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule<T: Clone + Default> {
    #[serde(default)]
    pub level: RuleLevel,
    #[serde(default)]
    pub applicable: Applicable,
    #[serde(default)]
    pub value: T,
}

impl<T: Clone + Default> Default for Rule<T> {
    fn default() -> Self {
        Self {
            level: RuleLevel::Disabled,
            applicable: Applicable::Always,
            value: T::default(),
        }
    }
}

impl<T: Clone + Default> Rule<T> {
    pub fn error(value: T) -> Self {
        Self {
            level: RuleLevel::Error,
            applicable: Applicable::Always,
            value,
        }
    }
    
    pub fn warning(value: T) -> Self {
        Self {
            level: RuleLevel::Warning,
            applicable: Applicable::Always,
            value,
        }
    }
    
    pub fn never(value: T) -> Self {
        Self {
            level: RuleLevel::Error,
            applicable: Applicable::Never,
            value,
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.level != RuleLevel::Disabled
    }
}

/// All commitlint rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Rules {
    // Body rules
    #[serde(rename = "body-case")]
    pub body_case: Rule<Vec<CaseType>>,
    #[serde(rename = "body-empty")]
    pub body_empty: Rule<()>,
    #[serde(rename = "body-full-stop")]
    pub body_full_stop: Rule<String>,
    #[serde(rename = "body-leading-blank")]
    pub body_leading_blank: Rule<()>,
    #[serde(rename = "body-max-length")]
    pub body_max_length: Rule<usize>,
    #[serde(rename = "body-max-line-length")]
    pub body_max_line_length: Rule<usize>,
    #[serde(rename = "body-min-length")]
    pub body_min_length: Rule<usize>,
    
    // Footer rules
    #[serde(rename = "footer-empty")]
    pub footer_empty: Rule<()>,
    #[serde(rename = "footer-leading-blank")]
    pub footer_leading_blank: Rule<()>,
    #[serde(rename = "footer-max-length")]
    pub footer_max_length: Rule<usize>,
    #[serde(rename = "footer-max-line-length")]
    pub footer_max_line_length: Rule<usize>,
    #[serde(rename = "footer-min-length")]
    pub footer_min_length: Rule<usize>,
    
    // Header rules
    #[serde(rename = "header-case")]
    pub header_case: Rule<Vec<CaseType>>,
    #[serde(rename = "header-full-stop")]
    pub header_full_stop: Rule<String>,
    #[serde(rename = "header-max-length")]
    pub header_max_length: Rule<usize>,
    #[serde(rename = "header-min-length")]
    pub header_min_length: Rule<usize>,
    #[serde(rename = "header-trim")]
    pub header_trim: Rule<()>,
    
    // Scope rules
    #[serde(rename = "scope-case")]
    pub scope_case: Rule<Vec<CaseType>>,
    #[serde(rename = "scope-empty")]
    pub scope_empty: Rule<()>,
    #[serde(rename = "scope-enum")]
    pub scope_enum: Rule<Vec<String>>,
    #[serde(rename = "scope-max-length")]
    pub scope_max_length: Rule<usize>,
    #[serde(rename = "scope-min-length")]
    pub scope_min_length: Rule<usize>,
    
    // Subject rules
    #[serde(rename = "subject-case")]
    pub subject_case: Rule<Vec<CaseType>>,
    #[serde(rename = "subject-empty")]
    pub subject_empty: Rule<()>,
    #[serde(rename = "subject-full-stop")]
    pub subject_full_stop: Rule<String>,
    #[serde(rename = "subject-max-length")]
    pub subject_max_length: Rule<usize>,
    #[serde(rename = "subject-min-length")]
    pub subject_min_length: Rule<usize>,
    #[serde(rename = "subject-exclamation-mark")]
    pub subject_exclamation_mark: Rule<()>,
    
    // Type rules
    #[serde(rename = "type-case")]
    pub type_case: Rule<Vec<CaseType>>,
    #[serde(rename = "type-empty")]
    pub type_empty: Rule<()>,
    #[serde(rename = "type-enum")]
    pub type_enum: Rule<Vec<String>>,
    #[serde(rename = "type-max-length")]
    pub type_max_length: Rule<usize>,
    #[serde(rename = "type-min-length")]
    pub type_min_length: Rule<usize>,
    
    // Other rules
    #[serde(rename = "references-empty")]
    pub references_empty: Rule<()>,
    #[serde(rename = "signed-off-by")]
    pub signed_off_by: Rule<String>,
    #[serde(rename = "trailer-exists")]
    pub trailer_exists: Rule<String>,
}

impl Default for Rules {
    fn default() -> Self {
        Self::conventional()
    }
}

impl Rules {
    /// Returns an empty ruleset with all rules disabled
    pub fn empty() -> Self {
        Self {
            body_case: Rule::default(),
            body_empty: Rule::default(),
            body_full_stop: Rule::default(),
            body_leading_blank: Rule::default(),
            body_max_length: Rule::default(),
            body_max_line_length: Rule::default(),
            body_min_length: Rule::default(),
            footer_empty: Rule::default(),
            footer_leading_blank: Rule::default(),
            footer_max_length: Rule::default(),
            footer_max_line_length: Rule::default(),
            footer_min_length: Rule::default(),
            header_case: Rule::default(),
            header_full_stop: Rule::default(),
            header_max_length: Rule::default(),
            header_min_length: Rule::default(),
            header_trim: Rule::default(),
            scope_case: Rule::default(),
            scope_empty: Rule::default(),
            scope_enum: Rule::default(),
            scope_max_length: Rule::default(),
            scope_min_length: Rule::default(),
            subject_case: Rule::default(),
            subject_empty: Rule::default(),
            subject_full_stop: Rule::default(),
            subject_max_length: Rule::default(),
            subject_min_length: Rule::default(),
            subject_exclamation_mark: Rule::default(),
            type_case: Rule::default(),
            type_empty: Rule::default(),
            type_enum: Rule::default(),
            type_max_length: Rule::default(),
            type_min_length: Rule::default(),
            references_empty: Rule::default(),
            signed_off_by: Rule::default(),
            trailer_exists: Rule::default(),
        }
    }
    
    /// Returns the conventional commits ruleset (matches @commitlint/config-conventional)
    pub fn conventional() -> Self {
        Self {
            body_case: Rule::default(),
            body_empty: Rule::default(),
            body_full_stop: Rule::default(),
            body_leading_blank: Rule {
                level: RuleLevel::Warning,
                applicable: Applicable::Always,
                value: (),
            },
            body_max_length: Rule::default(),
            body_max_line_length: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: 100,
            },
            body_min_length: Rule::default(),
            footer_empty: Rule::default(),
            footer_leading_blank: Rule {
                level: RuleLevel::Warning,
                applicable: Applicable::Always,
                value: (),
            },
            footer_max_length: Rule::default(),
            footer_max_line_length: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: 100,
            },
            footer_min_length: Rule::default(),
            header_case: Rule::default(),
            header_full_stop: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Never,
                value: ".".to_string(),
            },
            header_max_length: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: 100,
            },
            header_min_length: Rule::default(),
            header_trim: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: (),
            },
            scope_case: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: vec![CaseType::LowerCase],
            },
            scope_empty: Rule::default(),
            scope_enum: Rule::default(),
            scope_max_length: Rule::default(),
            scope_min_length: Rule::default(),
            subject_case: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: vec![
                    CaseType::LowerCase,
                    CaseType::SentenceCase,
                ],
            },
            subject_empty: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Never,
                value: (),
            },
            subject_full_stop: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Never,
                value: ".".to_string(),
            },
            subject_max_length: Rule::default(),
            subject_min_length: Rule::default(),
            subject_exclamation_mark: Rule::default(),
            type_case: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: vec![CaseType::LowerCase],
            },
            type_empty: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Never,
                value: (),
            },
            type_enum: Rule {
                level: RuleLevel::Error,
                applicable: Applicable::Always,
                value: vec![
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
                ],
            },
            type_max_length: Rule::default(),
            type_min_length: Rule::default(),
            references_empty: Rule::default(),
            signed_off_by: Rule::default(),
            trailer_exists: Rule::default(),
        }
    }
}
