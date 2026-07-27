use crate::config::Parser;
use regex::Regex;
use std::sync::LazyLock;

/// Matches the start of a footer line. Three accepted forms:
///
/// 1. The literal `BREAKING CHANGE:` prefix.
/// 2. A `token: value` git trailer, where `token` is an alphanumeric word
///    optionally hyphen-joined (`Reviewed-by:`, `Acked-by:`, `Refs:`). The
///    colon must be followed by a space, which is what keeps `https://…` from
///    being read as a trailer.
/// 3. A `token #value` reference, where the separator between the token and
///    `#` is either whitespace (`Closes #123`) or a colon optionally followed
///    by whitespace (`Fixes:#123`, `Fixes: #123`). The `#` must come
///    immediately after that separator, so a URL such as
///    `http://example.com/page#frag` cannot match — the path text sits between
///    the colon and the `#`.
static FOOTER_TRAILER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:BREAKING CHANGE:|[A-Za-z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)*: |[A-Za-z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)*(?::\s*|\s)#)",
    )
    .unwrap()
});

/// Pre-compiled regex for the default parser pattern, so the common case
/// avoids recompiling on every parse.
static DEFAULT_HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(crate::config::DEFAULT_PARSER_PATTERN).expect("default parser pattern is valid")
});

#[derive(Debug, Clone)]
pub struct ConventionalCommit {
    pub r#type: String,
    pub scope: Option<String>,
    pub breaking: bool,
    pub subject: String,
}

#[derive(Debug, Clone)]
pub struct CommitMessage {
    pub header: String,
    pub body: Option<String>,
    pub footer: Option<String>,
    /// True when the line immediately after the header is blank.
    pub body_has_leading_blank: bool,
    /// True when the line immediately before the first footer line is blank.
    pub footer_has_leading_blank: bool,
}

impl CommitMessage {
    pub fn from_str(msg: &str) -> Self {
        let lines: Vec<&str> = msg.lines().collect();
        let header = lines.first().map(|s| s.to_string()).unwrap_or_default();

        let mut body_lines = Vec::new();
        let mut footer_lines = Vec::new();
        let mut in_footer = false;
        let mut body_has_leading_blank = false;
        let mut footer_has_leading_blank = false;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if i == 1 && line.trim().is_empty() {
                body_has_leading_blank = true;
                continue; // Skip blank line after header
            }

            // Footer starts with BREAKING CHANGE or a git trailer like
            // "Reviewed-by:", "Acked-by:", "Closes #123", etc.
            if !in_footer && FOOTER_TRAILER_RE.is_match(line) {
                in_footer = true;
                // The blank separator line belongs to neither body nor footer
                if lines[i - 1].trim().is_empty() {
                    footer_has_leading_blank = true;
                    if body_lines
                        .last()
                        .map(|l: &&str| l.trim().is_empty())
                        .unwrap_or(false)
                    {
                        body_lines.pop();
                    }
                }
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
            header,
            body,
            footer,
            body_has_leading_blank,
            footer_has_leading_blank,
        }
    }

    pub fn parse_conventional(&self, parser: &Parser) -> anyhow::Result<ConventionalCommit> {
        // Resolve a commit field to its capture group name through the
        // correspondence map, falling back to the field name itself.
        fn group<'a>(parser: &'a Parser, field: &'a str) -> &'a str {
            parser
                .correspondence
                .get(field)
                .map(String::as_str)
                .unwrap_or(field)
        }

        /// Invariant for the `DEFAULT_HEADER_RE` fast path: the default pattern
        /// hard-codes its capture group names (`type`, `scope`, `breaking`,
        /// `subject`), so reusing the pre-compiled regex is only valid when the
        /// correspondence map is the identity mapping for those fields. With a
        /// renamed correspondence (e.g. `type = "kind"`) the lookup would ask
        /// for a group the default regex does not define, so such configs must
        /// take the compile branch instead.
        fn uses_default_groups(parser: &Parser) -> bool {
            ["type", "scope", "subject", "breaking"]
                .iter()
                .all(|field| group(parser, field) == *field)
        }

        // Declared outside the `if` so the compiled regex outlives the branch
        // and can be borrowed by `re`; moving it inside would force an
        // unconditional compile or fail to borrow-check.
        let compiled;
        let re: &Regex = if parser.pattern == crate::config::DEFAULT_PARSER_PATTERN
            && uses_default_groups(parser)
        {
            &DEFAULT_HEADER_RE
        } else {
            compiled = Regex::new(&parser.pattern)?;
            &compiled
        };

        if let Some(caps) = re.captures(&self.header) {
            let r#type = caps
                .name(group(parser, "type"))
                .map(|m| m.as_str().to_string())
                .ok_or_else(|| anyhow::anyhow!("Missing 'type' in commit message"))?;

            let scope = caps
                .name(group(parser, "scope"))
                .map(|m| m.as_str().to_string());
            let breaking = caps.name(group(parser, "breaking")).is_some();
            let subject = caps
                .name(group(parser, "subject"))
                .map(|m| m.as_str().to_string())
                .ok_or_else(|| anyhow::anyhow!("Missing 'subject' in commit message"))?;

            // Check footer for breaking change indicator. Conventional Commits
            // v1.0.0 treats "BREAKING-CHANGE:" as a required synonym of
            // "BREAKING CHANGE:", so both spellings must be recognised.
            let breaking_from_footer = self.footer.as_deref().is_some_and(|f| {
                f.lines()
                    .any(|l| l.starts_with("BREAKING CHANGE:") || l.starts_with("BREAKING-CHANGE:"))
            });

            Ok(ConventionalCommit {
                r#type,
                scope,
                breaking: breaking || breaking_from_footer,
                subject,
            })
        } else {
            anyhow::bail!("Commit message does not match conventional commit format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_simple_commit() {
        let msg = CommitMessage::from_str("feat: add new feature");
        assert_eq!(msg.header, "feat: add new feature");
    }

    #[test]
    fn test_parse_with_scope() {
        let msg = CommitMessage::from_str("feat(api): add endpoint");
        let commit = msg.parse_conventional(&Parser::default()).unwrap();
        assert_eq!(commit.r#type, "feat");
        assert_eq!(commit.scope, Some("api".to_string()));
    }

    #[test]
    fn test_parse_with_body() {
        let msg = CommitMessage::from_str("feat: add feature\n\nThis is the body");
        assert_eq!(msg.body, Some("This is the body".to_string()));
        assert!(msg.body_has_leading_blank);
    }

    #[test]
    fn test_parse_with_footer_trailers() {
        let msg = CommitMessage::from_str(
            "feat: add feature\n\nSome body text\n\nCloses #123\nReviewed-by: Alice",
        );
        assert_eq!(msg.body, Some("Some body text".to_string()));
        assert_eq!(
            msg.footer,
            Some("Closes #123\nReviewed-by: Alice".to_string())
        );
        assert!(msg.body_has_leading_blank);
        assert!(msg.footer_has_leading_blank);
    }

    #[test]
    fn test_parse_footer_without_leading_blank() {
        let msg = CommitMessage::from_str("feat: add feature\n\nSome body text\nCloses #123");
        assert_eq!(msg.body, Some("Some body text".to_string()));
        assert_eq!(msg.footer, Some("Closes #123".to_string()));
        assert!(!msg.footer_has_leading_blank);
    }

    #[test]
    fn test_parse_footer_reference_colon_without_space() {
        // "Fixes:#123" — colon immediately followed by "#", no space. This must
        // be recognised as a footer trailer, not swept into the body.
        let msg = CommitMessage::from_str("feat: add feature\n\nSome body text\n\nFixes:#123");
        assert_eq!(msg.footer, Some("Fixes:#123".to_string()));
        assert_eq!(msg.body, Some("Some body text".to_string()));
        assert!(
            !msg.body.as_deref().unwrap().contains("Fixes:#123"),
            "the trailer must not leak into the body"
        );
    }

    #[test]
    fn test_parse_footer_reference_separator_forms_agree() {
        // All three separator spellings land in the footer.
        for trailer in ["Closes #123", "Fixes:#123", "Fixes: #123"] {
            let msg =
                CommitMessage::from_str(&format!("feat: add feature\n\nSome body\n\n{trailer}"));
            assert_eq!(
                msg.footer,
                Some(trailer.to_string()),
                "{trailer} should parse as a footer trailer"
            );
            assert_eq!(msg.body, Some("Some body".to_string()));
        }
    }

    #[test]
    fn test_urls_in_body_are_not_footer_trailers() {
        // A URL has a colon but the "#" fragment is separated from it by the
        // rest of the URL, so the "token:#" form must not fire. Likewise a
        // scheme with no fragment at all.
        let msg = CommitMessage::from_str(
            "feat: add feature\n\nSee the discussion at\nhttp://example.com/page#frag\nand https://example.com/foo\nfor details",
        );
        assert_eq!(
            msg.body,
            Some(
                "See the discussion at\nhttp://example.com/page#frag\nand https://example.com/foo\nfor details"
                    .to_string()
            )
        );
        assert_eq!(msg.footer, None);
    }

    #[test]
    fn test_footer_trailer_re_rejects_urls() {
        // Direct assertions against the trailer regex, independent of how
        // `from_str` splits body from footer.
        for line in [
            "https://example.com/foo",
            "http://example.com/page#frag",
            "https://example.com/issues/123#comment",
        ] {
            assert!(
                !FOOTER_TRAILER_RE.is_match(line),
                "{line} must not match as a footer trailer"
            );
        }
        for line in ["Closes #123", "Fixes:#123", "Fixes: #123", "Refs:  #7"] {
            assert!(
                FOOTER_TRAILER_RE.is_match(line),
                "{line} must match as a footer trailer"
            );
        }
    }

    #[test]
    fn test_parse_breaking_change_marker() {
        let msg = CommitMessage::from_str("feat!: breaking change in API");
        let commit = msg.parse_conventional(&Parser::default()).unwrap();
        assert_eq!(commit.r#type, "feat");
        assert!(commit.breaking);
        assert_eq!(commit.subject, "breaking change in API");
    }

    #[test]
    fn test_parse_custom_pattern_with_correspondence() {
        let mut correspondence = HashMap::new();
        correspondence.insert("type".to_string(), "kind".to_string());
        correspondence.insert("scope".to_string(), "module".to_string());
        correspondence.insert("breaking".to_string(), "bang".to_string());
        correspondence.insert("subject".to_string(), "desc".to_string());
        let parser = Parser {
            pattern: r"^(?P<kind>\w+)(?:\((?P<module>[^)]+)\))?(?P<bang>!)?:\s(?P<desc>.*)$"
                .to_string(),
            correspondence,
        };

        let msg = CommitMessage::from_str("feat(api)!: add endpoint");
        let commit = msg.parse_conventional(&parser).unwrap();
        assert_eq!(commit.r#type, "feat");
        assert_eq!(commit.scope, Some("api".to_string()));
        assert!(commit.breaking);
        assert_eq!(commit.subject, "add endpoint");
    }

    /// Identity correspondence, i.e. what `Parser::default()` uses.
    fn default_correspondence() -> HashMap<String, String> {
        ["type", "scope", "subject", "breaking"]
            .iter()
            .map(|f| (f.to_string(), f.to_string()))
            .collect()
    }

    #[test]
    fn test_parse_breaking_change_footer_hyphen_synonym() {
        let msg = CommitMessage::from_str(
            "feat: add feature\n\nSome body text\n\nBREAKING-CHANGE: drops the old API",
        );
        assert_eq!(
            msg.footer,
            Some("BREAKING-CHANGE: drops the old API".to_string())
        );
        let commit = msg.parse_conventional(&Parser::default()).unwrap();
        assert!(commit.breaking);
    }

    #[test]
    fn test_parse_breaking_change_footer_space_form() {
        let msg = CommitMessage::from_str(
            "feat: add feature\n\nSome body text\n\nBREAKING CHANGE: drops the old API",
        );
        let commit = msg.parse_conventional(&Parser::default()).unwrap();
        assert!(commit.breaking);
    }

    #[test]
    fn test_parse_conventional_invalid_custom_pattern() {
        let parser = Parser {
            // Unclosed named group: must surface as an error, not a panic.
            pattern: "(?P<type".to_string(),
            correspondence: default_correspondence(),
        };

        let msg = CommitMessage::from_str("feat: add endpoint");
        assert!(msg.parse_conventional(&parser).is_err());
    }

    #[test]
    fn test_parse_default_pattern_with_remapped_correspondence() {
        // Default pattern plus a renamed correspondence: the pre-compiled fast
        // path must not be taken, because the default regex has no "kind"
        // group. The parse legitimately fails (the configured group does not
        // exist), but it must fail cleanly rather than panic or silently read
        // the wrong group. Rejecting this combination during config validation
        // is the proper long-term fix.
        let mut correspondence = default_correspondence();
        correspondence.insert("type".to_string(), "kind".to_string());
        let parser = Parser {
            pattern: crate::config::DEFAULT_PARSER_PATTERN.to_string(),
            correspondence,
        };

        let msg = CommitMessage::from_str("feat(api): add endpoint");
        let result = msg.parse_conventional(&parser);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("Missing 'type'"),
            "expected a missing-type error from the remapped correspondence"
        );
    }

    #[test]
    fn test_fast_and_slow_paths_agree() {
        // Semantically identical to DEFAULT_PARSER_PATTERN (`!` written as
        // `[!]`) but textually different, so it reaches the compile branch
        // instead of the pre-compiled DEFAULT_HEADER_RE.
        let equivalent = Parser {
            pattern:
                r"^(?P<type>\w+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>[!])?:\s(?P<subject>.*)$"
                    .to_string(),
            correspondence: default_correspondence(),
        };
        assert_ne!(equivalent.pattern, crate::config::DEFAULT_PARSER_PATTERN);

        for header in ["feat(api)!: add endpoint", "fix: correct typo"] {
            let msg = CommitMessage::from_str(header);
            let fast = msg.parse_conventional(&Parser::default()).unwrap();
            let slow = msg.parse_conventional(&equivalent).unwrap();
            assert_eq!(fast.r#type, slow.r#type);
            assert_eq!(fast.scope, slow.scope);
            assert_eq!(fast.subject, slow.subject);
            assert_eq!(fast.breaking, slow.breaking);
        }
    }
}
