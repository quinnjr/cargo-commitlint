# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-07-27

### Changed

- Dropped the `atty` dependency in favour of `std::io::IsTerminal`. `atty` is
  unmaintained and carried two RUSTSEC advisories (RUSTSEC-2024-0375 and the
  RUSTSEC-2021-0145 unaligned read); both are gone from the tree. Terminal
  detection behaves identically -- piped input is still read, and an interactive
  terminal still reports that no commit message was provided.
- Declared `rust-version = "1.80"`. That floor was already imposed by
  dependencies (`colored` 1.80, `clap` 1.74), so this documents the existing
  requirement rather than raising it.

### Fixed

- The shipped `commitlint.example.toml` could not be loaded at all. Rules that
  take no value of their own were written as `value = []` -- the natural TOML
  encoding of commitlint's `[2, "never"]` -- but that failed to deserialize and
  took the whole config down with it, so copying the example exactly as the docs
  instruct left the tool refusing to start. Value-less rules now accept and
  discard whatever is supplied.
- Long multibyte commit headers no longer crash the linter. The oversized-header
  display truncated by byte index, so a cut landing inside a character panicked
  (`end byte index 69 is not a char boundary`). Any sufficiently long accented or
  CJK header aborted with a Rust backtrace instead of reporting a lint result --
  in a commit-msg hook that meant a crash rather than a failed check. Truncation
  and measurement are now character-based in both the formatter and the validator.
- The generated commit-msg hook interpolated the resolved binary path into shell
  using double quotes, which do not neutralise `$`, backticks or quotes in POSIX
  `sh`. A repository checked out under a path containing them executed that path's
  contents on every commit. The path is now single-quoted with `'\''` escaping
  (CWE-78), covered by a regression test.
- The hook's fail-open warning goes to stderr, so editor and GUI git clients
  surface the notice when validation is skipped because no binary was found.
- `git2` no longer pulls in its default features. Only local `revwalk` and
  `find_commit` are used, so the libssh2 and OpenSSL network transports were
  compiled for nothing and broke builds in environments without that C toolchain.
- Resolved the lints that prevented this codebase from passing
  `cargo clippy -D warnings`: `or_insert_with(Vec::new)`, an identity `map_err`
  and a manual descending sort.

### Changed

- `main` now tracks this release lineage. The 2.0.0 rules engine was published
  but never merged, so `main` had continued on the 1.0.0-era codebase while CI,
  documentation and dependency work accumulated on top; the two are now one line.
- Hook installation uses `.commitlint/hooks` with the `build.rs` installer rather
  than cargo-husky.
- Repository, homepage and documentation URLs moved to the `quinnjr` owner.

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

[2.1.0]: https://github.com/quinnjr/cargo-commitlint/releases/tag/v2.1.0
[2.0.0]: https://github.com/quinnjr/cargo-commitlint/releases/tag/v2.0.0
[1.0.0]: https://github.com/quinnjr/cargo-commitlint/releases/tag/v1.0.0
