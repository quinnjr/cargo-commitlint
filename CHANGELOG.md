# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-01-04

### Added

#### Full commitlint Compatibility
- **Complete Rule Set**: All 30+ commitlint rules implemented with full parity
  - Type rules: `type-enum`, `type-case`, `type-empty`, `type-max-length`, `type-min-length`
  - Scope rules: `scope-enum`, `scope-case`, `scope-empty`, `scope-max-length`, `scope-min-length`
  - Subject rules: `subject-case`, `subject-empty`, `subject-full-stop`, `subject-max-length`, `subject-min-length`, `subject-exclamation-mark`
  - Header rules: `header-case`, `header-full-stop`, `header-max-length`, `header-min-length`, `header-trim`
  - Body rules: `body-case`, `body-empty`, `body-full-stop`, `body-leading-blank`, `body-max-length`, `body-max-line-length`, `body-min-length`
  - Footer rules: `footer-empty`, `footer-leading-blank`, `footer-max-length`, `footer-max-line-length`, `footer-min-length`
  - Other rules: `references-empty`, `signed-off-by`, `trailer-exists`

- **Rule Severity System**: Full commitlint-compatible rule configuration
  - Level 0: Disabled
  - Level 1: Warning
  - Level 2: Error
  - Applicability: `always` or `never` (inverts the rule)

#### CLI Enhancements
- **Git Log Linting**: Lint commits from git history
  - `--from <REF>`: Lower end of commit range (exclusive)
  - `--to <REF>`: Upper end of commit range (inclusive)
  - `--last`: Lint only the last commit
  - `--from-last-tag`: Use last tag as lower end of range
- **File Input**: `--edit [FILE]` to read from file or .git/COMMIT_EDITMSG
- **Environment Variable**: `--env <VAR>` to read from file at env var path
- **Output Formats**: `--format text|json|compact`
- **Colored Output**: `--color` flag (enabled by default)
- **Quiet Mode**: `--quiet` to suppress output on success
- **Verbose Mode**: `--verbose` to show output for valid commits
- **Strict Mode**: `--strict` for exit code 2 on warnings, 3 on errors
- **Help URL**: `--help-url` to display custom help URL in errors
- **Print Config**: `cargo commitlint print-config` to show resolved configuration

#### Multi-Format Configuration
- **TOML**: `commitlint.toml`, `.commitlint.toml`, `.commitlintrc.toml`
- **JSON**: `.commitlintrc.json`, `.commitlintrc`
- **YAML**: `.commitlintrc.yaml`, `.commitlintrc.yml`
- **package.json**: `"commitlint"` field support
- **Extends**: Support for `conventional` and `@commitlint/config-conventional` presets

#### Automatic Hook Installation (cargo-husky style)
- **Zero-Config Installation**: Hooks installed automatically on `cargo build`/`cargo test`
- **build.rs Integration**: No manual installation required
- **User-Hooks Mode**: Creates `.commitlint/hooks/` directory (can be committed to repo)
- **Git Config Integration**: Automatically sets `core.hooksPath`
- **Smart Hook Handling**: Appends to existing hooks instead of overwriting
- **CI-Aware**: Skips installation in CI environments
- **Cargo.toml Configuration**: Configure via `[package.metadata.commitlint]`

#### Environment Variables
- `COMMITLINT_SKIP`: Skip commit message validation
- `COMMITLINT_NO_INSTALL`: Skip automatic hook installation
- `COMMITLINT_USER_HOOKS`: Force user-hooks mode
- `COMMITLINT_INSTALL_IN_CI`: Enable installation in CI

### Changed

- **Breaking**: Configuration format changed to match commitlint
- **Breaking**: Rule configuration now uses `level`, `applicable`, and `value` fields
- Replaced cargo-husky dependency with built-in hook management
- Improved commit message parsing with better footer/trailer detection
- Enhanced reference extraction from commit body and footer

### Removed

- Removed cargo-husky dev-dependency (functionality now built-in)
- Removed old configuration format (migrated to commitlint-compatible format)

### Fixed

- Fixed `--edit` flag argument parsing
- Fixed duplicate `--config` argument in CLI
- Fixed YAML format detection for config files
- Fixed reference extraction from commit body

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

[2.0.0]: https://github.com/pegasusheavy/cargo-commitlint/releases/tag/v2.0.0
[1.0.0]: https://github.com/pegasusheavy/cargo-commitlint/releases/tag/v1.0.0
