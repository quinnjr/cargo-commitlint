# Contributing to cargo-commitlint

Thank you for your interest in contributing to cargo-commitlint! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/cargo-commitlint.git
   cd cargo-commitlint
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/quinnjr/cargo-commitlint.git
   ```

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Git Hooks

This project uses `cargo-husky` for git hooks. The hooks are automatically installed when you run:

```bash
cargo test
```

Available hooks:
- **pre-commit**: Runs `cargo fmt --check` and `cargo clippy`
- **pre-push**: Runs `cargo test`
- **commit-msg**: Validates commit messages using `cargo commitlint`

## Making Changes

1. **Create a branch** from `develop`:
   ```bash
   git checkout develop
   git pull upstream develop
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the project's coding standards

3. **Write tests** for new functionality

4. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --check
   ```

5. **Commit your changes** following Conventional Commits:
   ```bash
   git commit -m "feat: add new feature"
   ```

## Commit Message Guidelines

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages must follow this format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `build`: Build system changes
- `ci`: CI configuration changes
- `chore`: Other changes

### Examples

```
feat: add support for custom commit types
fix: resolve issue with scope validation
docs: update installation instructions
refactor: simplify error handling logic
```

## Pull Request Process

1. **Update your branch** with the latest changes:
   ```bash
   git fetch upstream
   git rebase upstream/develop
   ```

2. **Push your changes** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request** on GitHub:
   - Use a clear, descriptive title
   - Fill out the pull request template
   - Link any related issues
   - Ensure all CI checks pass

4. **Respond to feedback** and make requested changes

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Follow clippy suggestions (`cargo clippy`)
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and small
- Write meaningful commit messages

## Testing

- Write tests for new functionality
- Ensure all existing tests pass
- Aim for good test coverage
- Test edge cases and error conditions

## Documentation

- Update documentation for new features
- Keep code comments up to date
- Update examples if behavior changes
- Follow the existing documentation style

## Reporting Issues

When reporting issues, please use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml) and include:

- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Relevant configuration or code

## Requesting Features

When requesting features, please use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml) and include:

- Problem statement
- Proposed solution
- Use cases
- Alternatives considered

## Questions

For questions, use the [question template](.github/ISSUE_TEMPLATE/question.yml) or check:

- [README.md](README.md)
- [Documentation](https://quinnjr.github.io/cargo-commitlint/)
- Existing issues and discussions

## Release Process

Releases are managed by maintainers. The process includes:

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag
4. GitHub Actions handles the release

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT License).

## Getting Help

- Open an issue for bugs or feature requests
- Check existing issues and discussions
- Review the documentation

Thank you for contributing to cargo-commitlint! 🦀

