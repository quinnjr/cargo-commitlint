use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConventionalCommit {
    pub r#type: String,
    pub scope: Option<String>,
    pub breaking: bool,
    pub subject: String,
    pub body: Option<String>,
    pub footer: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct CommitMessage {
    pub raw: String,
    pub header: String,
    pub body: Option<String>,
    pub footer: Option<String>,
}

impl CommitMessage {
    pub fn from_str(msg: &str) -> Self {
        let lines: Vec<&str> = msg.lines().collect();
        let header = lines.first().map(|s| s.to_string()).unwrap_or_default();

        let mut body_lines = Vec::new();
        let mut footer_lines = Vec::new();
        let mut in_footer = false;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if i == 1 && line.trim().is_empty() {
                continue; // Skip blank line after header
            }

            // Footer starts with BREAKING CHANGE or a token like "Closes:", "Fixes:", etc.
            if line.starts_with("BREAKING CHANGE:")
                || Regex::new(r"^[A-Z][a-z]+(?:-[A-Z][a-z]+)*:")
                    .unwrap()
                    .is_match(line)
            {
                in_footer = true;
            }

            if in_footer {
                footer_lines.push(*line);
            } else {
                body_lines.push(*line);
            }
        }

        let body = if body_lines.is_empty() {
            None
        } else {
            Some(body_lines.join("\n"))
        };

        let footer = if footer_lines.is_empty() {
            None
        } else {
            Some(footer_lines.join("\n"))
        };

        Self {
            raw: msg.to_string(),
            header,
            body,
            footer,
        }
    }

    pub fn parse_conventional(&self, pattern: &str) -> anyhow::Result<ConventionalCommit> {
        let re = Regex::new(pattern)?;

        if let Some(caps) = re.captures(&self.header) {
            let r#type = caps
                .name("type")
                .map(|m| m.as_str().to_string())
                .ok_or_else(|| anyhow::anyhow!("Missing 'type' in commit message"))?;

            let scope = caps.name("scope").map(|m| m.as_str().to_string());
            let breaking = caps.name("breaking").is_some();
            let subject = caps
                .name("subject")
                .map(|m| m.as_str().to_string())
                .ok_or_else(|| anyhow::anyhow!("Missing 'subject' in commit message"))?;

            // Parse footer for breaking changes and other metadata
            let mut footer_map = HashMap::new();
            if let Some(ref footer) = self.footer {
                for line in footer.lines() {
                    if line.starts_with("BREAKING CHANGE:") {
                        footer_map.insert(
                            "BREAKING CHANGE".to_string(),
                            line.strip_prefix("BREAKING CHANGE:")
                                .unwrap_or("")
                                .trim()
                                .to_string(),
                        );
                    } else if let Some((key, value)) = line.split_once(':') {
                        footer_map.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
            }

            // Check footer for breaking change indicator
            let breaking_from_footer = footer_map.contains_key("BREAKING CHANGE");

            Ok(ConventionalCommit {
                r#type,
                scope,
                breaking: breaking || breaking_from_footer,
                subject,
                body: self.body.clone(),
                footer: if footer_map.is_empty() {
                    None
                } else {
                    Some(footer_map)
                },
            })
        } else {
            anyhow::bail!("Commit message does not match conventional commit format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_commit() {
        let msg = CommitMessage::from_str("feat: add new feature");
        assert_eq!(msg.header, "feat: add new feature");
    }

    #[test]
    fn test_parse_with_scope() {
        let msg = CommitMessage::from_str("feat(api): add endpoint");
        let pattern = r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s(?P<subject>.*)$";
        let commit = msg.parse_conventional(pattern).unwrap();
        assert_eq!(commit.r#type, "feat");
        assert_eq!(commit.scope, Some("api".to_string()));
    }

    #[test]
    fn test_parse_with_body() {
        let msg = CommitMessage::from_str("feat: add feature\n\nThis is the body");
        assert_eq!(msg.body, Some("This is the body".to_string()));
    }
}
