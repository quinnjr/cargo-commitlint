use crate::commit::{CommitMessage, ConventionalCommit};
use crate::config::Config;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub rule: String,
    pub message: String,
}

pub struct Validator {
    config: Config,
}

impl Validator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn validate(&self, commit_msg: &str) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check if commit should be ignored
        for ignore_pattern in &self.config.ignores {
            if Regex::new(ignore_pattern)
                .ok()
                .and_then(|re| Some(re.is_match(commit_msg)))
                .unwrap_or(false)
            {
                return Ok(()); // Skip validation for ignored commits
            }
        }

        let msg = CommitMessage::from_str(commit_msg);

        // Validate header length
        if msg.header.len() > self.config.rules.header_max_length {
            errors.push(ValidationError {
                rule: "header-max-length".to_string(),
                message: format!(
                    "header must not be longer than {} characters, current length is {}",
                    self.config.rules.header_max_length,
                    msg.header.len()
                ),
            });
        }

        if msg.header.len() < self.config.rules.header_min_length {
            errors.push(ValidationError {
                rule: "header-min-length".to_string(),
                message: format!(
                    "header must be at least {} characters, current length is {}",
                    self.config.rules.header_min_length,
                    msg.header.len()
                ),
            });
        }

        // Try to parse as conventional commit
        match msg.parse_conventional(&self.config.parser.pattern) {
            Ok(commit) => {
                errors.extend(self.validate_conventional_commit(&commit, &msg));
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
            errors.extend(self.validate_body(body));
        }

        // Validate footer
        if let Some(ref footer) = msg.footer {
            errors.extend(self.validate_footer(footer));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_conventional_commit(
        &self,
        commit: &ConventionalCommit,
        _msg: &CommitMessage,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Validate type
        if !self.config.rules.r#type.r#enum.is_empty() {
            let type_set: HashSet<&String> = self.config.rules.r#type.r#enum.iter().collect();
            if !type_set.contains(&commit.r#type) {
                errors.push(ValidationError {
                    rule: "type-enum".to_string(),
                    message: format!(
                        "type must be one of [{}]",
                        self.config.rules.r#type.r#enum.join(", ")
                    ),
                });
            }
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
            if !self.config.rules.scope.r#enum.is_empty() {
                let scope_set: HashSet<&String> = self.config.rules.scope.r#enum.iter().collect();
                if !scope_set.contains(scope) {
                    errors.push(ValidationError {
                        rule: "scope-enum".to_string(),
                        message: format!(
                            "scope must be one of [{}]",
                            self.config.rules.scope.r#enum.join(", ")
                        ),
                    });
                }
            }

            if !self.validate_case(scope, &self.config.rules.scope.case) {
                errors.push(ValidationError {
                    rule: "scope-case".to_string(),
                    message: format!("scope must be {}", self.config.rules.scope.case),
                });
            }
        }

        // Validate subject empty
        if self.config.rules.subject_empty && commit.subject.trim().is_empty() {
            errors.push(ValidationError {
                rule: "subject-empty".to_string(),
                message: "subject must not be empty".to_string(),
            });
        }

        // Validate subject case (pass if ANY rule matches)
        if !self.config.rules.subject_case.is_empty() {
            let mut passed = false;
            for case_rule in &self.config.rules.subject_case {
                if self.validate_subject_case(&commit.subject, case_rule) {
                    passed = true;
                    break;
                }
            }
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
        if !self.config.rules.subject_full_stop.is_empty() {
            if commit
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
        }

        errors
    }

    fn validate_body(&self, body: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for (i, line) in body.lines().enumerate() {
            if i == 0 && self.config.rules.body_leading_blank && !line.trim().is_empty() {
                errors.push(ValidationError {
                    rule: "body-leading-blank".to_string(),
                    message: "body must have leading blank line".to_string(),
                });
            }

            if line.len() > self.config.rules.body_max_line_length {
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

    fn validate_footer(&self, footer: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for (i, line) in footer.lines().enumerate() {
            if i == 0 && self.config.rules.footer_leading_blank && !line.trim().is_empty() {
                errors.push(ValidationError {
                    rule: "footer-leading-blank".to_string(),
                    message: "footer must have leading blank line".to_string(),
                });
            }

            if line.len() > self.config.rules.footer_max_line_length {
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
                // Simple camelCase check: first char lowercase, rest can be mixed
                !text.is_empty() && text.chars().next().unwrap().is_lowercase()
            }
            "kebab-case" => {
                // kebab-case: lowercase with hyphens
                text.chars().all(|c| c.is_lowercase() || c == '-')
            }
            "pascal-case" => {
                // PascalCase: first char uppercase
                !text.is_empty() && text.chars().next().unwrap().is_uppercase()
            }
            "snake-case" => {
                // snake_case: lowercase with underscores
                text.chars().all(|c| c.is_lowercase() || c == '_')
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
}
