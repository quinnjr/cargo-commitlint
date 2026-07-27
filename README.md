# cargo-commitlint

A Rust-based commit message linter following the [Conventional Commits](https://www.conventionalcommits.org/) specification. Similar to Node.js's `commitlint`, but written entirely in Rust and designed for the Rust ecosystem.

## Features

- ✅ Validates commit messages against Conventional Commits specification
- ✅ Configurable via TOML (similar to `commitlint`)
- ✅ Git hook integration (similar to `cargo-husky`)
- ✅ Supports all standard Conventional Commit types
- ✅ Customizable rules for type, scope, subject, body, and footer
- ✅ Regex-based commit message parsing
- ✅ Ignore patterns for skipping validation

## Installation

### From Source

```bash
git clone https://github.com/quinnjr/cargo-commitlint.git
cd cargo-commitlint
cargo install --path .
```

### From Crates.io (when published)

```bash
cargo install cargo-commitlint
```

## Usage

After installation, `cargo-commitlint` is available as a cargo subcommand. Use it with:

```bash
cargo commitlint <command>
```

### Git Hooks with cargo-husky

This project uses `cargo-husky` to manage git hooks. The hooks are automatically installed when you run `cargo test`.

#### Available Hooks

- **pre-commit**: Runs `cargo fmt --check` and `cargo clippy` to ensure code quality
- **pre-push**: Runs `cargo test` to ensure all tests pass before pushing
- **commit-msg**: Validates commit messages using `cargo commitlint` and Conventional Commits

The **commit-msg** hook only validates once a `cargo-commitlint` binary exists — either built in this repository with `cargo build --release` or installed with `cargo install cargo-commitlint`. If no binary is found, the hook prints a warning to stderr and skips validation instead of blocking the commit, so build the binary at least once on a fresh clone.

#### Installing Hooks

Simply run:

```bash
cargo test
```

This will compile the project and install all git hooks configured in `.cargo-husky/hooks/`.

#### Manual Git Hook Installation (Alternative)

If you prefer to use `cargo commitlint`'s built-in hook installer instead of `cargo-husky`:

```bash
cargo commitlint install
```

This will create a `.git/hooks/commit-msg` hook that validates all commit messages using `cargo commitlint`. Like the bundled hook, the generated hook validates only while a `cargo-commitlint` binary is present; if the binary is missing (after `cargo clean`, for example), it warns on stderr and lets the commit through unvalidated.

### Uninstall Git Hook

Remove the git hook:

```bash
cargo commitlint uninstall
```

### Validate Commit Messages

Validate a commit message directly:

```bash
# Validate from command line
cargo commitlint check --message "feat: add new feature"

# Validate from stdin
echo "feat: add new feature" | cargo commitlint check
```

### Configuration

Create a `commitlint.toml` or `.commitlint.toml` file in your project root. You can copy `commitlint.example.toml` as a starting point:

```bash
cp commitlint.example.toml commitlint.toml
```

#### Example Configuration

```toml
# Ignore patterns (regex) — top-level keys must come before any [table]
ignores = [
    # "Merge.*",
    # "Revert.*",
]

[rules]
# Subject validation
subject_case = ["sentence-case"]
subject_empty = false
subject_full_stop = "."

# Header validation
header_max_length = 72
header_min_length = 0

# Body validation
body_leading_blank = true
body_max_line_length = 100

# Footer validation
footer_leading_blank = true
footer_max_line_length = 100

# Whether breaking-change commits (`!` or `BREAKING CHANGE:` footer) are allowed
allow_breaking = true

# Type validation
[rules.type]
enum = ["feat", "fix", "docs", "style", "refactor", "test", "chore"]
case = "lowercase"

# Scope validation
[rules.scope]
enum = []  # Empty means all scopes allowed
case = "lowercase"

# Parser configuration
[parser]
pattern = "^(?P<type>\\w+)(?:\\((?P<scope>[^)]+)\\))?(?P<breaking>!)?:\\s(?P<subject>.*)$"
```

## Conventional Commits Format

The tool validates commit messages in the following format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `build`: Changes that affect the build system or external dependencies
- `ci`: Changes to CI configuration files and scripts
- `chore`: Other changes that don't modify src or test files
- `revert`: Reverts a previous commit

### Examples

Valid commit messages:

```
feat: add user authentication
feat(api): add new endpoint
fix: resolve memory leak in parser
docs: update README with installation instructions
feat!: breaking change in API

feat: add feature

This is a longer description of the change.

Closes #123
```

Invalid commit messages:

```
invalid: bad commit type
feat:Missing space after colon
feat: subject too long and exceeds the maximum length limit of 72 characters
```

- `invalid: bad commit type` — `type-enum`: `invalid` is not one of the allowed types
- `feat:Missing space after colon` — the parser pattern requires `:` followed by whitespace, so the header does not match at all
- `feat: subject too long ...` — `header-max-length`: the header is 76 characters, over the 72 default

## Configuration Options

### Rules

- `rules.type.enum`: List of allowed commit types (empty = all allowed)
- `rules.type.case`: Case requirement (`lowercase`, `uppercase`, `camel-case`, `kebab-case`, `pascal-case`, `snake-case`)
- `rules.scope.enum`: List of allowed scopes (empty = all allowed)
- `rules.scope.case`: Case requirement for scope
- `rules.subject_case`: List of allowed case formats (`sentence-case`, `lowercase`, `uppercase`, `start-case`). Note: `sentence-case` is permissive — it accepts any subject that starts with a letter or digit
- `rules.subject_empty`: Whether subject can be empty
- `rules.subject_full_stop`: Character that should not appear at end of subject
- `rules.header_max_length`: Maximum header length
- `rules.header_min_length`: Minimum header length
- `rules.body_leading_blank`: Require blank line before body
- `rules.body_max_line_length`: Maximum line length in body
- `rules.footer_leading_blank`: Require blank line before footer
- `rules.footer_max_line_length`: Maximum line length in footer
- `rules.allow_breaking`: Whether breaking-change commits (`!` marker or `BREAKING CHANGE:` footer) are allowed (default `true`)

### Parser

- `parser.pattern`: Regex pattern for parsing conventional commits
- `parser.correspondence`: Map regex capture groups to commit fields

### Ignores

- `ignores`: List of regex patterns for commits to skip validation

## Integration with Cargo

This tool is designed to work seamlessly with Rust projects and integrates with `cargo-husky` for comprehensive git hook management.

### Using cargo-husky (Recommended)

This project includes `cargo-husky` configuration with pre-commit, pre-push, and commit-msg hooks:

1. **Pre-commit hook**: Ensures code is formatted (`cargo fmt`) and passes clippy checks
2. **Pre-push hook**: Runs all tests before allowing pushes
3. **Commit-msg hook**: Validates commit messages using `cargo commitlint` (skipped with a warning on stderr if no `cargo-commitlint` binary has been built or installed)

To activate the hooks, simply run:

```bash
cargo test
```

The hooks are defined in `.cargo-husky/hooks/` and will be automatically installed.

### Using cargo commitlint's Built-in Hook Installer

Alternatively, you can use `cargo commitlint`'s built-in installer:

```bash
cargo commitlint install
```

This will install only the commit-msg hook for commit message validation. As with the bundled hook, validation is skipped with a warning on stderr whenever no `cargo-commitlint` binary can be found.

## License

Licensed under the MIT License.

Copyright (c) 2025 Pegasus Heavy Industries LLC

See [LICENSE](LICENSE) or [LICENSE-MIT](LICENSE-MIT) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

