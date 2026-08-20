# cargo-commitlint

A Rust-based commit message linter following the [Conventional Commits](https://www.conventionalcommits.org/) specification. Similar to Node.js's `commitlint`, but written entirely in Rust and designed for the Rust ecosystem.

## Features

- ✅ Full commitlint rule set — 36 rules with severity levels and always/never applicability
- ✅ Configurable via TOML, JSON or YAML (`.commitlintrc` variants supported)
- ✅ Lints git history, not just one message (`--last`, `--from`, `--to`, `--from-last-tag`)
- ✅ Output as text, JSON or compact for CI consumption
- ✅ Git hook installation, with a build script for dev-dependency use
- ✅ Ignore patterns, plus built-in ignores for merge and revert commits

## Installation

### From Source

```bash
git clone https://github.com/quinnjr/cargo-commitlint.git
cd cargo-commitlint
cargo install --path .
```

### From Crates.io

```bash
cargo install cargo-commitlint
```

## Usage

After installation, `cargo-commitlint` is available as a cargo subcommand. Use it with:

```bash
cargo commitlint <command>
```

### Git Hooks

Install the commit-msg hook into the current repository:

```bash
cargo commitlint install
```

That writes `.git/hooks/commit-msg`, which validates every commit message. Pass
`--force` to replace a hook that is already there.

When `cargo-commitlint` is a dev-dependency, the build script installs the hook
for you. Configure it under `[package.metadata.commitlint]` in `Cargo.toml`:

```toml
[package.metadata.commitlint]
# Write hooks to .commitlint/hooks/ so they can be committed to the repo
user-hooks = true
# no-install = true   # disable automatic installation entirely
```

Set `COMMITLINT_SKIP=1` to bypass the hook for a single commit.

If no `cargo-commitlint` binary can be found, the hook prints a warning to
stderr and lets the commit through rather than blocking it, so build or install
the binary at least once on a fresh clone.

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
# Copy to commitlint.toml, .commitlint.toml, .commitlintrc.toml or
# .cargo/commitlint.toml. JSON and YAML variants (.commitlintrc.json,
# .commitlintrc.yaml) work too.

# Skip validation for commits matching these regex patterns
ignores = ["^Merge branch", "^Revert "]

# Built-in ignores for merge, revert and squash commits
defaultIgnores = true

# Every rule takes the same three fields:
#   level       0 = disabled, 1 = warning, 2 = error
#   applicable  "always", or "never" to invert the rule
#   value       rule-specific; omit it (or use []) when the rule takes none

[rules.type-enum]
level = 2
applicable = "always"
value = ["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"]

[rules.type-case]
level = 2
applicable = "always"
value = ["lower-case"]

# "never" inverts it: the type must NOT be empty
[rules.type-empty]
level = 2
applicable = "never"

[rules.subject-case]
level = 2
applicable = "always"
value = ["lower-case", "sentence-case"]

[rules.header-max-length]
level = 2
applicable = "always"
value = 100

# level 1 reports a warning without failing the commit
[rules.body-leading-blank]
level = 1
applicable = "always"

[parser]
header_pattern = "^(?P<type>\\w+)(?:\\((?P<scope>[^)]+)\\))?(?P<breaking>!)?:\\s*(?P<subject>.*)$"
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
Feat: capitalised type
feat: ends with a period.
no conventional prefix at all
```

- `invalid: bad commit type` — `type-enum`: not one of the allowed types
- `Feat: capitalised type` — `type-case` (and `type-enum`): types must be lower-case
- `feat: ends with a period.` — `subject-full-stop` and `header-full-stop`
- `no conventional prefix at all` — `type-empty` and `subject-empty`: the header does not parse

Run `cargo commitlint print-config` to see which rules are active and at what level.

## Configuration Options

Configuration is commitlint-compatible. Every rule is configured the same way:

```toml
[rules.<rule-name>]
level = 2            # 0 = disabled, 1 = warning, 2 = error
applicable = "always" # or "never", which inverts the rule
value = ...          # rule-specific; omit it (or use []) when the rule takes none
```

`applicable = "never"` inverts the check — `type-empty` with `never` means the
type must *not* be empty.

### Available rules

- **Type**: `type-enum`, `type-case`, `type-empty`, `type-max-length`, `type-min-length`
- **Scope**: `scope-enum`, `scope-case`, `scope-empty`, `scope-max-length`, `scope-min-length`
- **Subject**: `subject-case`, `subject-empty`, `subject-full-stop`, `subject-max-length`, `subject-min-length`, `subject-exclamation-mark`
- **Header**: `header-case`, `header-full-stop`, `header-max-length`, `header-min-length`, `header-trim`
- **Body**: `body-case`, `body-empty`, `body-full-stop`, `body-leading-blank`, `body-max-length`, `body-max-line-length`, `body-min-length`
- **Footer**: `footer-empty`, `footer-leading-blank`, `footer-max-length`, `footer-max-line-length`, `footer-min-length`
- **Other**: `references-empty`, `signed-off-by`, `trailer-exists`

Case values are `lower-case`, `upper-case`, `camel-case`, `kebab-case`,
`pascal-case`, `sentence-case`, `snake-case`, `start-case`.

Run `cargo commitlint print-config` to see every rule with its resolved default.

### Top-level keys

- `rules`: the rule table described above
- `parser.header_pattern`: regex used to split the header
- `parser.header_correspondence`: names of the pattern's capture groups
- `parser.note_keywords`: breaking-change trailers (`BREAKING CHANGE`, `BREAKING-CHANGE`)
- `parser.reference_actions`: issue-closing keywords (`closes`, `fixes`, ...)
- `ignores`: regex patterns for commits to skip entirely
- `defaultIgnores`: skip merge, revert and squash commits (default `true`)
- `extends`: inherit from a preset, e.g. `["conventional"]`
- `helpUrl`: URL shown in error output

## Integration with Cargo

Add `cargo-commitlint` as a dev-dependency and its build script installs the
commit-msg hook when the crate is built:

```toml
[dev-dependencies]
cargo-commitlint = "2"

[package.metadata.commitlint]
user-hooks = true
```

`user-hooks = true` writes the hook to `.commitlint/hooks/` so it can be
committed and shared; point git at it once with:

```bash
git config core.hooksPath .commitlint/hooks
```

Set `no-install = true` to disable automatic installation, or use
`cargo commitlint install` to manage the hook by hand.

### Linting history in CI

```bash
# every commit since the last tag
cargo commitlint check --from-last-tag

# a range, or just the most recent commit
cargo commitlint check --from origin/main --to HEAD
cargo commitlint check --last

# machine-readable output
cargo commitlint check --last --format json
```

## License

Licensed under the MIT License.

Copyright (c) 2025 Joseph R. Quinn

See [LICENSE](LICENSE) or [LICENSE-MIT](LICENSE-MIT) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

