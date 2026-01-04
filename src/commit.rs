//! Commit message parsing module
//!
//! Parses commit messages following the Conventional Commits specification:
//! https://www.conventionalcommits.org/

use regex::Regex;
use std::collections::HashMap;

/// A parsed conventional commit
#[derive(Debug, Clone, Default)]
pub struct ConventionalCommit {
    /// Raw commit message
    pub raw: String,
    /// Commit header (first line)
    pub header: String,
    /// Commit type (feat, fix, etc.)
    pub commit_type: Option<String>,
    /// Commit scope (optional, in parentheses)
    pub scope: Option<String>,
    /// Whether this is a breaking change (! after type/scope)
    pub breaking: bool,
    /// Commit subject (description after type:)
    pub subject: Option<String>,
    /// Commit body (optional, after blank line)
    pub body: Option<String>,
    /// Commit footer (optional, contains trailers)
    pub footer: Option<String>,
    /// Breaking change description (from BREAKING CHANGE: trailer)
    pub breaking_change: Option<String>,
    /// Issue references (e.g., #123, closes #456)
    pub references: Vec<Reference>,
    /// All trailers/notes
    pub notes: Vec<Note>,
    /// All footer trailers as key-value pairs
    pub trailers: HashMap<String, Vec<String>>,
}

/// An issue reference
#[derive(Debug, Clone)]
pub struct Reference {
    pub action: Option<String>,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub issue: String,
    pub raw: String,
}

/// A note/trailer in the commit footer
#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub text: String,
}

impl ConventionalCommit {
    /// Parse a commit message using the given header pattern
    pub fn parse(message: &str, header_pattern: &str) -> Self {
        let mut commit = ConventionalCommit {
            raw: message.to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = message.lines().collect();

        if lines.is_empty() {
            return commit;
        }

        // Parse header
        commit.header = lines[0].to_string();
        commit.parse_header(header_pattern);

        // Parse body and footer
        if lines.len() > 1 {
            commit.parse_body_and_footer(&lines[1..]);
        }

        commit
    }

    /// Parse the header line
    fn parse_header(&mut self, pattern: &str) {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(&self.header) {
                self.commit_type = caps.name("type").map(|m| m.as_str().to_string());
                self.scope = caps.name("scope").map(|m| m.as_str().to_string());
                self.breaking = caps.name("breaking").is_some();
                self.subject = caps.name("subject").map(|m| m.as_str().to_string());
            }
        }
    }

    /// Parse body and footer sections
    fn parse_body_and_footer(&mut self, lines: &[&str]) {
        let mut body_lines: Vec<&str> = Vec::new();
        let mut footer_lines: Vec<&str> = Vec::new();
        let mut in_footer = false;
        let mut had_blank = false;

        // Footer trailer pattern: "Token: value" or "Token #value" or "BREAKING CHANGE: value"
        let trailer_re = Regex::new(r"^([A-Za-z][A-Za-z0-9-]*|BREAKING CHANGE):\s*(.*)$").unwrap();
        let breaking_re = Regex::new(r"^BREAKING[ -]CHANGE:\s*(.*)$").unwrap();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                had_blank = true;
                if !in_footer && !body_lines.is_empty() {
                    body_lines.push(*line);
                }
                continue;
            }

            // Check if this line starts a footer
            if had_blank && (trailer_re.is_match(line) || breaking_re.is_match(line)) {
                in_footer = true;
            }

            if in_footer {
                footer_lines.push(*line);
            } else {
                body_lines.push(*line);
            }
        }

        // Set body
        if !body_lines.is_empty() {
            // Remove leading blank line if present
            let body_start = if body_lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
                1
            } else {
                0
            };

            let body_content: Vec<&str> = body_lines[body_start..].to_vec();
            if !body_content.is_empty() {
                self.body = Some(body_content.join("\n"));
            }
        }

        // Parse footer
        if !footer_lines.is_empty() {
            self.footer = Some(footer_lines.join("\n"));
            self.parse_footer(&footer_lines);
        }

        // Also extract references from body (they might not be in proper footer format)
        if let Some(body) = self.body.clone() {
            self.extract_references_from_text(&body);
        }
    }

    /// Extract issue references from text
    fn extract_references_from_text(&mut self, text: &str) {
        let reference_re = Regex::new(r"(?i)(close[sd]?|fix(?:e[sd])?|resolve[sd]?)\s+#(\d+)").unwrap();
        let issue_re = Regex::new(r"#(\d+)").unwrap();

        // Extract action references (e.g., "Fixes #123")
        for caps in reference_re.captures_iter(text) {
            let action = caps.get(1).map(|m| m.as_str().to_lowercase());
            let issue = caps.get(2).unwrap().as_str().to_string();

            // Avoid duplicates
            if !self.references.iter().any(|r| r.issue == issue) {
                self.references.push(Reference {
                    action,
                    owner: None,
                    repository: None,
                    issue: issue.clone(),
                    raw: format!("#{}", issue),
                });
            }
        }

        // Extract bare references (e.g., "#123")
        for caps in issue_re.captures_iter(text) {
            let issue = caps.get(1).unwrap().as_str().to_string();
            // Avoid duplicates
            if !self.references.iter().any(|r| r.issue == issue) {
                self.references.push(Reference {
                    action: None,
                    owner: None,
                    repository: None,
                    issue: issue.clone(),
                    raw: format!("#{}", issue),
                });
            }
        }
    }

    /// Parse footer trailers
    fn parse_footer(&mut self, lines: &[&str]) {
        let trailer_re = Regex::new(r"^([A-Za-z][A-Za-z0-9-]*|BREAKING CHANGE):\s*(.*)$").unwrap();
        let reference_re = Regex::new(r"(?i)(close[sd]?|fix(?:e[sd])?|resolve[sd]?)\s+#(\d+)").unwrap();
        let issue_re = Regex::new(r"#(\d+)").unwrap();

        let mut current_trailer: Option<(String, String)> = None;

        for line in lines {
            if let Some(caps) = trailer_re.captures(line) {
                // Save previous trailer
                if let Some((key, value)) = current_trailer.take() {
                    self.add_trailer(&key, &value);
                }

                let key = caps.get(1).unwrap().as_str().to_string();
                let value = caps.get(2).unwrap().as_str().to_string();
                current_trailer = Some((key, value));
            } else if let Some((ref _key, ref mut value)) = current_trailer {
                // Continuation of previous trailer
                if !value.is_empty() {
                    value.push('\n');
                }
                value.push_str(line);
            }
        }

        // Save last trailer
        if let Some((key, value)) = current_trailer {
            self.add_trailer(&key, &value);
        }

        // Extract BREAKING CHANGE
        if let Some(breaking_values) = self.trailers.get("BREAKING CHANGE") {
            self.breaking_change = breaking_values.first().cloned();
            if self.breaking_change.is_some() {
                self.breaking = true;
            }
        }
        if let Some(breaking_values) = self.trailers.get("BREAKING-CHANGE") {
            self.breaking_change = breaking_values.first().cloned();
            if self.breaking_change.is_some() {
                self.breaking = true;
            }
        }

        // Extract references from footer
        let footer_text = lines.join("\n");
        for caps in reference_re.captures_iter(&footer_text) {
            let action = caps.get(1).map(|m| m.as_str().to_lowercase());
            let issue = caps.get(2).unwrap().as_str().to_string();
            self.references.push(Reference {
                action,
                owner: None,
                repository: None,
                issue: issue.clone(),
                raw: format!("#{}", issue),
            });
        }

        // Also find bare references
        for caps in issue_re.captures_iter(&footer_text) {
            let issue = caps.get(1).unwrap().as_str().to_string();
            // Avoid duplicates
            if !self.references.iter().any(|r| r.issue == issue) {
                self.references.push(Reference {
                    action: None,
                    owner: None,
                    repository: None,
                    issue: issue.clone(),
                    raw: format!("#{}", issue),
                });
            }
        }

        // Create notes from trailers
        for (key, values) in &self.trailers {
            for value in values {
                self.notes.push(Note {
                    title: key.clone(),
                    text: value.clone(),
                });
            }
        }
    }

    /// Add a trailer to the trailers map
    fn add_trailer(&mut self, key: &str, value: &str) {
        self.trailers
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.trim().to_string());
    }

    /// Check if the commit has a valid conventional format
    pub fn is_valid(&self) -> bool {
        self.commit_type.is_some() && self.subject.is_some()
    }

    /// Get the full description (subject + body)
    pub fn full_description(&self) -> String {
        match &self.body {
            Some(body) => format!("{}\n\n{}", self.subject.as_deref().unwrap_or(""), body),
            None => self.subject.clone().unwrap_or_default(),
        }
    }
}

/// Parse multiple commit messages (e.g., from git log)
pub fn parse_commits(messages: &[String], header_pattern: &str) -> Vec<ConventionalCommit> {
    messages
        .iter()
        .map(|msg| ConventionalCommit::parse(msg, header_pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_PATTERN: &str = r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s*(?P<subject>.*)$";

    #[test]
    fn test_simple_commit() {
        let commit = ConventionalCommit::parse("feat: add new feature", DEFAULT_PATTERN);
        assert_eq!(commit.commit_type, Some("feat".to_string()));
        assert_eq!(commit.subject, Some("add new feature".to_string()));
        assert_eq!(commit.scope, None);
        assert!(!commit.breaking);
    }

    #[test]
    fn test_commit_with_scope() {
        let commit = ConventionalCommit::parse("feat(api): add endpoint", DEFAULT_PATTERN);
        assert_eq!(commit.commit_type, Some("feat".to_string()));
        assert_eq!(commit.scope, Some("api".to_string()));
        assert_eq!(commit.subject, Some("add endpoint".to_string()));
    }

    #[test]
    fn test_breaking_change_marker() {
        let commit = ConventionalCommit::parse("feat!: breaking change", DEFAULT_PATTERN);
        assert!(commit.breaking);
    }

    #[test]
    fn test_commit_with_body() {
        let msg = "feat: add feature\n\nThis is the body\nwith multiple lines";
        let commit = ConventionalCommit::parse(msg, DEFAULT_PATTERN);
        assert_eq!(commit.body, Some("This is the body\nwith multiple lines".to_string()));
    }

    #[test]
    fn test_commit_with_footer() {
        let msg = "feat: add feature\n\nBody text\n\nCloses: #123\nReviewed-by: John";
        let commit = ConventionalCommit::parse(msg, DEFAULT_PATTERN);
        assert!(commit.footer.is_some());
        assert!(commit.trailers.contains_key("Closes"));
        assert!(commit.trailers.contains_key("Reviewed-by"));
    }

    #[test]
    fn test_breaking_change_footer() {
        let msg = "feat: add feature\n\nBREAKING CHANGE: This breaks everything";
        let commit = ConventionalCommit::parse(msg, DEFAULT_PATTERN);
        assert!(commit.breaking);
        assert_eq!(commit.breaking_change, Some("This breaks everything".to_string()));
    }

    #[test]
    fn test_references() {
        let msg = "fix: bug fix\n\nFixes #123\nCloses #456";
        let commit = ConventionalCommit::parse(msg, DEFAULT_PATTERN);
        assert!(!commit.references.is_empty());
    }
}
