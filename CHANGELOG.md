# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features
- Support for more commit types
- Enhanced error messages
- Performance optimizations

## [2.0.0] - 2026-07-22

### Added

- `rules.allow_breaking` config option (default `true`) with a `breaking-not-allowed` rule to reject breaking-change commits (`!` marker or `BREAKING CHANGE:`/`BREAKING-CHANGE:` footer)
- `parser.correspondence` is now honored, mapping regex capture groups to commit fields
- Broader footer/trailer detection: `Reviewed-by:`, `Acked-by:`, and both `Closes #123` and `Fixes:#123` reference forms are now recognized
- `Config::validate()` checks every user-supplied regex (`ignores` entries and `parser.pattern`) once at load time, reporting all bad patterns in a single error
- Substantially expanded test suite (6 → 74 Rust tests, 2 → 11 docs-site tests) covering every documented rule

### Changed

- **BREAKING:** `subject_empty` now matches its documented meaning — with the default `subject_empty = false`, an empty subject is rejected. Previously the check was inverted and never fired by default
- **BREAKING:** an explicit `--config <path>` that does not exist is now an error (exit 1) instead of silently falling back to defaults
- **BREAKING:** header/body/footer length limits now count characters rather than bytes, so multibyte commit messages are measured correctly
- **BREAKING:** an invalid regex in `ignores` or `parser.pattern` now aborts with exit 1 before the commit message is read. Previously it warned on stderr, was silently dropped, and let the commit through — so a typo meant the intended skip never happened while the warning reappeared on every commit
- Git hooks (both the bundled cargo-husky hook and the one written by `cargo commitlint install`) now invoke the resolved binary directly instead of routing through `cargo` as a subcommand, and skip with a warning on stderr when no binary is found instead of building the project on the commit path
- `cargo commitlint install` now backs up a pre-existing foreign `commit-msg` hook instead of overwriting it, and `uninstall` restores it
- Documentation and package URLs moved from `pegasusheavy` to `quinnjr`

### Fixed

- `body_leading_blank` and `footer_leading_blank` no longer fail every multi-line commit — the blank-line separators are now tracked during parsing instead of being tested against already-stripped text
- `commitlint.example.toml` (and the README/docs examples) had invalid TOML structure: scalar rule keys placed after `[rules.scope]` were silently assigned to the wrong table, and a top-level `ignores` array after `[parser.correspondence]` made the file fail to parse outright
- An invalid `parser.pattern` regex is now reported as a config error (`parser-pattern-invalid`) rather than as a bogus `type-enum` commit-type error
- Case validators: `kebab-case`/`snake-case` now accept digits; `camel-case`/`pascal-case` now reject separators
- Sitemap sub-page URLs pointed at a nonexistent host
- Release workflow published GitHub releases with no attached binaries

### Security

- The `commit-msg` hook written by `cargo commitlint install` interpolated the binary path into the generated shell script using double quotes, which do not neutralize `$`, backticks, or embedded quotes in POSIX `sh`. A repository checked out under a path containing shell metacharacters would execute them on every commit. The path is now emitted in single quotes with proper escaping (CWE-78)

## [1.0.0] - 2025-12-15

### Added

#### Core Features
- **Commit Message Validation**: Full support for Conventional Commits specification validation
- **TOML Configuration**: Configurable rules via `commitlint.toml` or `.commitlint.toml` files
- **Git Hook Integration**: Built-in installer for git commit-msg hooks
- **Cargo Subcommand**: Works seamlessly as `cargo commitlint` after installation
- **Multiple Validation Rules**: Support for type, scope, subject, body, and footer validation
- **Case Validation**: Support for various case formats (lowercase, uppercase, sentence-case, etc.)
- **Regex-based Parsing**: Flexible commit message parsing with customizable patterns
- **Ignore Patterns**: Skip validation for specific commit patterns using regex

#### Configuration Options
- Type validation with enum and case requirements
- Scope validation with enum and case requirements
- Subject validation (case, empty check, full stop)
- Header validation (min/max length)
- Body validation (leading blank, max line length)
- Footer validation (leading blank, max line length)
- Custom parser patterns
- Ignore patterns for skipping validation

#### CLI Commands
- `cargo commitlint install` - Install git commit-msg hook
- `cargo commitlint uninstall` - Remove git commit-msg hook
- `cargo commitlint check` - Validate commit messages (with `--message` flag or stdin)

#### Documentation
- Comprehensive documentation site built with Angular
- Getting Started guide
- Configuration reference
- Examples and use cases
- API reference
- Contributing guidelines
- Conventional Commits specification guide

#### Developer Experience
- Integration with `cargo-husky` for comprehensive git hook management
- Pre-commit hook for code formatting and clippy checks
- Pre-push hook for running tests
- Commit-msg hook for automatic commit message validation

#### SEO & Discoverability
- `llms.txt` file for LLM crawlers
- `ai.txt` file for AI crawlers
- Comprehensive meta tags (Open Graph, Twitter Cards)
- Structured data (JSON-LD)
- XML sitemap
- robots.txt with AI crawler support

#### CI/CD
- GitHub Actions workflows for CI
- Multi-platform release workflow (Linux, Windows, macOS)
- Documentation deployment workflow
- CodeQL security scanning
- Automated dependency updates with Dependabot
- Stale issue/PR management

#### Project Infrastructure
- Issue templates (bug report, feature request, question)
- Pull request template
- Security policy
- Contributing guidelines
- Cursor rules for development workflow

### Changed

- Initial release

### Fixed

- Cargo subcommand argument handling for proper `cargo commitlint` usage

### Security

- Security audit workflow in CI
- CodeQL analysis for vulnerability detection
- Security policy for responsible disclosure

---

[2.0.0]: https://github.com/quinnjr/cargo-commitlint/releases/tag/v2.0.0
[1.0.0]: https://github.com/quinnjr/cargo-commitlint/releases/tag/v1.0.0

