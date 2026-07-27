use crate::commit::{CommitMessage, ConventionalCommit};
use crate::config::Config;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub rule: String,
    pub message: String,
}

pub struct Validator {
    config: Config,
    ignore_patterns: Vec<Regex>,
    /// Set when the user supplied a custom `parser.pattern` that is not a valid
    /// regex, so the problem is reported as a configuration error instead of as
    /// a bogus per-commit rule violation.
    parser_pattern_error: Option<String>,
}

impl Validator {
    pub fn new(config: Config) -> Self {
        let ignore_patterns = config
            .ignores
            .iter()
            .filter_map(|pattern| match Regex::new(pattern) {
                Ok(re) => Some(re),
                Err(e) => {
                    eprintln!("warning: invalid ignore pattern '{}': {}", pattern, e);
                    None
                }
            })
            .collect();

        // Check a custom `parser.pattern` up front so an invalid regex is
        // diagnosed as a config problem rather than failing every commit with a
        // misleading `type-enum` error. Accepted trade-off: users with a *valid*
        // custom pattern pay one extra regex compile per process (the compiled
        // regex is discarded here; `parse_conventional` still owns compilation).
        // Users on the default pattern are unaffected -- they hit the
        // pre-compiled `LazyLock` static. Correct diagnosis is worth it.
        let parser_pattern_error = if config.parser.pattern == crate::config::DEFAULT_PARSER_PATTERN
        {
            None
        } else {
            match Regex::new(&config.parser.pattern) {
                Ok(_) => None,
                Err(e) => Some(format!("invalid parser.pattern regex: {}", e)),
            }
        };

        Self {
            config,
            ignore_patterns,
            parser_pattern_error,
        }
    }

    pub fn validate(&self, commit_msg: &str) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check if commit should be ignored
        for re in &self.ignore_patterns {
            if re.is_match(commit_msg) {
                return Ok(()); // Skip validation for ignored commits
            }
        }

        // A broken custom `parser.pattern` makes every downstream check
        // meaningless, so report the config error and stop here.
        if let Some(ref message) = self.parser_pattern_error {
            errors.push(ValidationError {
                rule: "parser-pattern-invalid".to_string(),
                message: message.clone(),
            });
            return Err(errors);
        }

        let msg = CommitMessage::from_str(commit_msg);

        // Validate header length
        let header_length = msg.header.chars().count();
        if header_length > self.config.rules.header_max_length {
            errors.push(ValidationError {
                rule: "header-max-length".to_string(),
                message: format!(
                    "header must not be longer than {} characters, current length is {}",
                    self.config.rules.header_max_length, header_length
                ),
            });
        }

        if header_length < self.config.rules.header_min_length {
            errors.push(ValidationError {
                rule: "header-min-length".to_string(),
                message: format!(
                    "header must be at least {} characters, current length is {}",
                    self.config.rules.header_min_length, header_length
                ),
            });
        }

        // Try to parse as conventional commit
        match msg.parse_conventional(&self.config.parser) {
            Ok(commit) => {
                errors.extend(self.validate_conventional_commit(&commit));
            }
            Err(e) => {
                errors.push(ValidationError {
                    rule: "type-enum".to_string(),
                    message: format!("Invalid conventional commit format: {}", e),
                });
            }
        }

        // Validate body
        if let Some(ref body) = msg.body {
            errors.extend(self.validate_body(body, msg.body_has_leading_blank));
        }

        // Validate footer
        if let Some(ref footer) = msg.footer {
            errors.extend(self.validate_footer(footer, msg.footer_has_leading_blank));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_conventional_commit(&self, commit: &ConventionalCommit) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Validate type
        if !self.config.rules.r#type.r#enum.is_empty()
            && !self.config.rules.r#type.r#enum.contains(&commit.r#type)
        {
            errors.push(ValidationError {
                rule: "type-enum".to_string(),
                message: format!(
                    "type must be one of [{}]",
                    self.config.rules.r#type.r#enum.join(", ")
                ),
            });
        }

        // Validate type case
        if !self.validate_case(&commit.r#type, &self.config.rules.r#type.case) {
            errors.push(ValidationError {
                rule: "type-case".to_string(),
                message: format!("type must be {}", self.config.rules.r#type.case),
            });
        }

        // Validate scope
        if let Some(ref scope) = commit.scope {
            if !self.config.rules.scope.r#enum.is_empty()
                && !self.config.rules.scope.r#enum.contains(scope)
            {
                errors.push(ValidationError {
                    rule: "scope-enum".to_string(),
                    message: format!(
                        "scope must be one of [{}]",
                        self.config.rules.scope.r#enum.join(", ")
                    ),
                });
            }

            if !self.validate_case(scope, &self.config.rules.scope.case) {
                errors.push(ValidationError {
                    rule: "scope-case".to_string(),
                    message: format!("scope must be {}", self.config.rules.scope.case),
                });
            }
        }

        // Validate breaking change allowance
        if !self.config.rules.allow_breaking && commit.breaking {
            errors.push(ValidationError {
                rule: "breaking-not-allowed".to_string(),
                message: "breaking changes are not allowed".to_string(),
            });
        }

        // Validate subject empty
        if !self.config.rules.subject_empty && commit.subject.trim().is_empty() {
            errors.push(ValidationError {
                rule: "subject-empty".to_string(),
                message: "subject must not be empty".to_string(),
            });
        }

        // Validate subject case (pass if ANY rule matches)
        if !self.config.rules.subject_case.is_empty() {
            let passed = self
                .config
                .rules
                .subject_case
                .iter()
                .any(|case_rule| self.validate_subject_case(&commit.subject, case_rule));
            if !passed {
                errors.push(ValidationError {
                    rule: "subject-case".to_string(),
                    message: format!(
                        "subject must match one of: {}",
                        self.config.rules.subject_case.join(", ")
                    ),
                });
            }
        }

        // Validate subject full stop
        if !self.config.rules.subject_full_stop.is_empty()
            && commit
                .subject
                .ends_with(&self.config.rules.subject_full_stop)
        {
            errors.push(ValidationError {
                rule: "subject-full-stop".to_string(),
                message: format!(
                    "subject must not end with '{}'",
                    self.config.rules.subject_full_stop
                ),
            });
        }

        errors
    }

    fn validate_body(&self, body: &str, has_leading_blank: bool) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.config.rules.body_leading_blank && !has_leading_blank {
            errors.push(ValidationError {
                rule: "body-leading-blank".to_string(),
                message: "body must have leading blank line".to_string(),
            });
        }

        for (i, line) in body.lines().enumerate() {
            if line.chars().count() > self.config.rules.body_max_line_length {
                errors.push(ValidationError {
                    rule: "body-max-line-length".to_string(),
                    message: format!(
                        "body line {} must not be longer than {} characters",
                        i + 1,
                        self.config.rules.body_max_line_length
                    ),
                });
            }
        }

        errors
    }

    fn validate_footer(&self, footer: &str, has_leading_blank: bool) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.config.rules.footer_leading_blank && !has_leading_blank {
            errors.push(ValidationError {
                rule: "footer-leading-blank".to_string(),
                message: "footer must have leading blank line".to_string(),
            });
        }

        for (i, line) in footer.lines().enumerate() {
            if line.chars().count() > self.config.rules.footer_max_line_length {
                errors.push(ValidationError {
                    rule: "footer-max-line-length".to_string(),
                    message: format!(
                        "footer line {} must not be longer than {} characters",
                        i + 1,
                        self.config.rules.footer_max_line_length
                    ),
                });
            }
        }

        errors
    }

    fn validate_case(&self, text: &str, case: &str) -> bool {
        match case {
            "lowercase" => text.chars().all(|c| !c.is_uppercase()),
            "uppercase" => text.chars().all(|c| !c.is_lowercase()),
            "camel-case" => {
                // camelCase: first char lowercase, remaining chars alphanumeric (no separators)
                let mut chars = text.chars();
                chars.next().is_some_and(|c| c.is_lowercase()) && chars.all(|c| c.is_alphanumeric())
            }
            "kebab-case" => {
                // kebab-case: lowercase letters, digits, and hyphens
                text.chars()
                    .all(|c| c.is_lowercase() || c.is_numeric() || c == '-')
            }
            "pascal-case" => {
                // PascalCase: first char uppercase, remaining chars alphanumeric (no separators)
                let mut chars = text.chars();
                chars.next().is_some_and(|c| c.is_uppercase()) && chars.all(|c| c.is_alphanumeric())
            }
            "snake-case" => {
                // snake_case: lowercase letters, digits, and underscores
                text.chars()
                    .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
            }
            _ => true, // Unknown case, skip validation
        }
    }

    fn validate_subject_case(&self, subject: &str, case_rule: &str) -> bool {
        match case_rule {
            "lowercase" => subject.chars().all(|c| !c.is_uppercase()),
            "uppercase" => subject.chars().all(|c| !c.is_lowercase()),
            "sentence-case" => {
                // Sentence case: typically first char uppercase, but lowercase is common in commits
                // Be lenient and allow both to match commitlint's practical behavior
                if subject.is_empty() {
                    return true;
                }
                // Allow lowercase subjects (very common in commit messages)
                // Also allow proper sentence case (uppercase first letter)
                let first_char = subject.chars().next().unwrap();
                first_char.is_lowercase() || first_char.is_uppercase() || first_char.is_numeric()
            }
            "start-case" => {
                // Start Case: Each Word Starts With Capital
                subject.split_whitespace().all(|word| {
                    word.chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false)
                })
            }
            _ => true, // Unknown case, skip validation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn has_rule(errors: &[ValidationError], rule: &str) -> bool {
        errors.iter().any(|e| e.rule == rule)
    }

    /// Build a `Validator` from the default config with `mutate` applied to the
    /// rules, so tests only spell out the rule they actually care about.
    fn validator_with(mutate: impl FnOnce(&mut crate::config::Rules)) -> Validator {
        let mut rules = Config::default().rules;
        mutate(&mut rules);
        Validator::new(Config {
            rules,
            ..Config::default()
        })
    }

    #[test]
    fn test_validate_valid_commit() {
        let config = Config::default();
        let validator = Validator::new(config);
        let result = validator.validate("feat: add new feature");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_type() {
        let config = Config::default();
        let validator = Validator::new(config);
        let result = validator.validate("invalid: add feature");
        assert!(result.is_err());
    }

    #[test]
    fn test_header_max_length_violation() {
        let validator = Validator::new(Config::default());
        let msg = format!("feat: {}", "a".repeat(80));
        let errors = validator.validate(&msg).unwrap_err();
        assert!(has_rule(&errors, "header-max-length"));
    }

    #[test]
    fn test_header_min_length_violation() {
        let validator = validator_with(|r| r.header_min_length = 30);
        let errors = validator.validate("feat: short").unwrap_err();
        assert!(has_rule(&errors, "header-min-length"));
    }

    #[test]
    fn test_subject_full_stop_violation() {
        let validator = Validator::new(Config::default());
        let errors = validator.validate("feat: add feature.").unwrap_err();
        assert!(has_rule(&errors, "subject-full-stop"));
    }

    #[test]
    fn test_subject_empty_rejected_by_default() {
        let validator = Validator::new(Config::default());
        let errors = validator.validate("feat: ").unwrap_err();
        assert!(has_rule(&errors, "subject-empty"));
    }

    #[test]
    fn test_breaking_allowed_by_default() {
        let validator = Validator::new(Config::default());
        assert!(validator.validate("feat!: change the API").is_ok());
    }

    #[test]
    fn test_breaking_marker_rejected_when_disallowed() {
        let validator = validator_with(|r| r.allow_breaking = false);
        let errors = validator.validate("feat!: change the API").unwrap_err();
        assert!(has_rule(&errors, "breaking-not-allowed"));
    }

    #[test]
    fn test_breaking_footer_rejected_when_disallowed() {
        let validator = validator_with(|r| r.allow_breaking = false);
        let msg =
            "feat: change the API\n\nSome body text.\n\nBREAKING CHANGE: the API is different now";
        let errors = validator.validate(msg).unwrap_err();
        assert!(has_rule(&errors, "breaking-not-allowed"));
    }

    #[test]
    fn test_breaking_footer_rejected_when_disallowed_no_body() {
        let validator = validator_with(|r| r.allow_breaking = false);
        let msg = "feat: change the API\n\nBREAKING CHANGE: the API is different now";
        let errors = validator.validate(msg).unwrap_err();
        assert!(has_rule(&errors, "breaking-not-allowed"));
    }

    #[test]
    fn test_subject_empty_allowed_when_configured() {
        let validator = validator_with(|r| r.subject_empty = true);
        assert!(validator.validate("feat: ").is_ok());
    }

    #[test]
    fn test_type_enum_rejection() {
        let validator = Validator::new(Config::default());
        let errors = validator.validate("invalid: add feature").unwrap_err();
        assert!(has_rule(&errors, "type-enum"));
    }

    #[test]
    fn test_type_case_violation() {
        let validator = Validator::new(Config::default());
        let errors = validator.validate("FEAT: add feature").unwrap_err();
        assert!(has_rule(&errors, "type-case"));
    }

    #[test]
    fn test_scope_enum_rejection() {
        let validator = validator_with(|r| r.scope.r#enum = vec!["api".to_string()]);
        let errors = validator.validate("feat(core): add feature").unwrap_err();
        assert!(has_rule(&errors, "scope-enum"));
    }

    #[test]
    fn test_scope_case_violation() {
        let validator = Validator::new(Config::default());
        let errors = validator.validate("feat(API): add feature").unwrap_err();
        assert!(has_rule(&errors, "scope-case"));
    }

    #[test]
    fn test_body_leading_blank_pass() {
        let validator = Validator::new(Config::default());
        assert!(validator
            .validate("feat: add feature\n\nThis is the body")
            .is_ok());
    }

    #[test]
    fn test_body_leading_blank_fail() {
        let validator = Validator::new(Config::default());
        let errors = validator
            .validate("feat: add feature\nThis is the body")
            .unwrap_err();
        assert!(has_rule(&errors, "body-leading-blank"));
    }

    #[test]
    fn test_body_max_line_length_violation() {
        let validator = Validator::new(Config::default());
        let msg = format!("feat: add feature\n\n{}", "a".repeat(120));
        let errors = validator.validate(&msg).unwrap_err();
        assert!(has_rule(&errors, "body-max-line-length"));
    }

    #[test]
    fn test_footer_leading_blank_pass() {
        let validator = Validator::new(Config::default());
        assert!(validator
            .validate("feat: add feature\n\nSome body text\n\nCloses #123")
            .is_ok());
    }

    #[test]
    fn test_footer_leading_blank_fail() {
        let validator = Validator::new(Config::default());
        let errors = validator
            .validate("feat: add feature\n\nSome body text\nCloses #123")
            .unwrap_err();
        assert!(has_rule(&errors, "footer-leading-blank"));
    }

    #[test]
    fn test_footer_max_line_length_violation() {
        let validator = Validator::new(Config::default());
        let msg = format!(
            "feat: add feature\n\nSome body text\n\nRefs: {}",
            "a".repeat(120)
        );
        let errors = validator.validate(&msg).unwrap_err();
        assert!(has_rule(&errors, "footer-max-line-length"));
    }

    #[test]
    fn test_ignores_skip_validation() {
        let config = Config {
            ignores: vec!["^Merge".to_string()],
            ..Config::default()
        };
        let validator = Validator::new(config);
        assert!(validator
            .validate("Merge branch 'main' into develop")
            .is_ok());
    }

    #[test]
    fn test_invalid_commit_format() {
        let validator = Validator::new(Config::default());
        let errors = validator
            .validate("this is not a conventional commit")
            .unwrap_err();
        assert!(has_rule(&errors, "type-enum"));
    }

    #[test]
    fn test_invalid_ignore_pattern_is_dropped_not_fatal() {
        let config = Config {
            ignores: vec!["[".to_string()],
            ..Config::default()
        };
        // An unparseable ignore pattern must be discarded with a warning, not
        // panic and not act as a blanket skip.
        let validator = Validator::new(config);
        let errors = validator.validate("nonsense message").unwrap_err();
        assert!(has_rule(&errors, "type-enum"));
    }

    #[test]
    fn test_invalid_parser_pattern_reports_distinct_rule() {
        let config = Config {
            parser: crate::config::Parser {
                pattern: "(?P<type".to_string(),
                ..Default::default()
            },
            ..Config::default()
        };
        let validator = Validator::new(config);
        let errors = validator.validate("feat: add new feature").unwrap_err();
        assert!(has_rule(&errors, "parser-pattern-invalid"));
        assert!(!has_rule(&errors, "type-enum"));
    }

    #[test]
    fn test_valid_custom_parser_pattern_still_validates() {
        let config = Config {
            parser: crate::config::Parser {
                // Same shape as the default pattern but with `\s+`, so it takes
                // the custom-pattern branch.
                pattern:
                    r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s+(?P<subject>.*)$"
                        .to_string(),
                ..Default::default()
            },
            ..Config::default()
        };
        let validator = Validator::new(config);
        let result = validator.validate("feat: add new feature");
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
        let errors = validator.validate("invalid: add feature").unwrap_err();
        assert!(has_rule(&errors, "type-enum"));
        assert!(!has_rule(&errors, "parser-pattern-invalid"));
    }

    #[test]
    fn test_header_max_length_counts_chars_not_bytes() {
        let max = Config::default().rules.header_max_length;
        // "feat: " is 6 chars; the rest is 'é' at 2 bytes per char, so the
        // header lands exactly on the limit in chars but well over it in bytes.
        let subject_len = max - "feat: ".chars().count();
        let at_limit = format!("feat: {}", "é".repeat(subject_len));
        assert_eq!(at_limit.chars().count(), max);
        assert!(at_limit.len() > max, "fixture must be multibyte");
        let result = Validator::new(Config::default()).validate(&at_limit);
        assert!(result.is_ok(), "expected Ok, got {:?}", result);

        let over_limit = format!("feat: {}", "é".repeat(subject_len + 1));
        let errors = Validator::new(Config::default())
            .validate(&over_limit)
            .unwrap_err();
        assert!(has_rule(&errors, "header-max-length"));
    }

    #[test]
    fn test_header_min_length_boundary_equality_passes() {
        let msg = "feat: short";
        assert_eq!(msg.chars().count(), 11);
        let validator = validator_with(|r| r.header_min_length = 11);
        let result = validator.validate(msg);
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[test]
    fn test_case_validators_via_scope_case() {
        // scope.enum stays empty by default so only the case check applies.
        let camel = validator_with(|r| r.scope.case = "camel-case".to_string());
        assert!(camel.validate("feat(fooBar): x").is_ok());
        let errors = camel.validate("feat(foo-bar): x").unwrap_err();
        assert!(has_rule(&errors, "scope-case"));

        let pascal = validator_with(|r| r.scope.case = "pascal-case".to_string());
        assert!(pascal.validate("feat(FooBar): x").is_ok());
        let errors = pascal.validate("feat(Foo-Bar): x").unwrap_err();
        assert!(has_rule(&errors, "scope-case"));

        let kebab = validator_with(|r| r.scope.case = "kebab-case".to_string());
        assert!(kebab.validate("feat(foo-bar2): x").is_ok());

        let snake = validator_with(|r| r.scope.case = "snake-case".to_string());
        assert!(snake.validate("feat(foo_bar2): x").is_ok());
    }

    #[test]
    fn test_multi_char_subject_full_stop() {
        let validator = validator_with(|r| r.subject_full_stop = "!!".to_string());
        let errors = validator.validate("feat: wow!!").unwrap_err();
        assert!(has_rule(&errors, "subject-full-stop"));

        let result = validator.validate("feat: wow!");
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[test]
    fn test_readme_multiline_example_is_valid() {
        let validator = Validator::new(Config::default());
        let result = validator.validate(
            "feat: add feature\n\nThis is a longer description of the change.\n\nCloses #123",
        );
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }
}
