<!-- converted from Cursor rules -->

## Cursor rule: `.cursor/rules/auto-commit.mdc`

# Auto-Commit After Completion

## Rule
Always commit changes after completing a task or set of changes, unless explicitly told otherwise by the user.

## Guidelines
- After completing a task, create a git commit with an appropriate commit message
- Use Conventional Commits format for commit messages (enforced by cargo-commitlint)
- Commit message should describe what was done, not how it was done
- Use appropriate commit type: feat, fix, docs, refactor, chore, etc.
- Include scope when relevant to the change
- Commit should be atomic - group related changes together

## Commit Message Format
Follow Conventional Commits specification:
- `feat(scope): description` - for new features
- `fix(scope): description` - for bug fixes
- `docs(scope): description` - for documentation changes
- `refactor(scope): description` - for code refactoring
- `chore(scope): description` - for maintenance tasks
- `test(scope): description` - for test additions/changes

## When to Commit
- After implementing a feature or fix
- After making configuration changes
- After updating dependencies
- After refactoring code
- After adding or updating documentation
- After completing a set of related changes

## When NOT to Commit
- If the user explicitly asks not to commit
- If the changes are incomplete or broken
- If the user wants to review changes first
- If committing would break the build or tests

## Best Practices
- Write clear, descriptive commit messages
- Keep commits focused on a single change or related set of changes
- Ensure code compiles and tests pass before committing
- Use `git add` to stage specific files, not `git add .` unless all changes should be committed


## Cursor rule: `.cursor/rules/git-flow.mdc`

# Git Flow Workflow

## Rule
This project uses git-flow branching model for version control and release management.

## Branch Structure
- **main/master**: Production-ready code, always deployable
- **develop**: Integration branch for features, where development happens
- **feature/**: Feature branches branched from and merged back to develop
- **release/**: Release branches for preparing new releases, branched from develop
- **hotfix/**: Hotfix branches for urgent production fixes, branched from main/master
- **support/**: Support branches for older versions (if needed)

## Workflow Guidelines
- Always branch from `develop` when starting new features
- Feature branches should follow naming: `feature/description-of-feature`
- Release branches should follow naming: `release/x.y.z` (version number)
- Hotfix branches should follow naming: `hotfix/description-of-fix`
- Merge feature branches back to `develop` when complete
- Create release branches from `develop` when ready to release
- Merge release branches to both `main` and `develop` when complete
- Hotfixes branch from `main`, merge back to both `main` and `develop`

## Commit Messages
- Follow Conventional Commits specification (enforced by cargo-commitlint)
- Use appropriate types: feat, fix, docs, style, refactor, perf, test, chore, etc.
- Include scope when relevant: `feat(parser): add new feature`

## Best Practices
- Keep feature branches focused on a single feature or fix
- Regularly merge `develop` into feature branches to stay up to date
- Delete branches after merging
- Tag releases on `main` branch with version numbers
- Use pull requests/merge requests for code review before merging


## Cursor rule: `.cursor/rules/no-unnecessary-markdown.mdc`

# No Unnecessary Markdown Generation

## Rule
Do not generate unnecessary markdown files or markdown content unless explicitly requested by the user.

## Guidelines
- Do not create markdown documentation files (README.md, CHANGELOG.md, etc.) unless explicitly requested
- Do not generate markdown summaries of tasks or changes unless the user asks for them
- Do not create markdown comments or documentation blocks unless they serve a specific purpose
- Focus on code implementation rather than documentation unless documentation is the explicit goal
- When providing explanations or summaries, use plain text in responses rather than generating markdown files

## Exceptions
- README.md updates are acceptable when they are part of implementing a feature that requires documentation
- Code comments and doc comments in source files are always acceptable and encouraged
- Markdown in code (like Rust doc comments) is acceptable as it's part of the code itself


## Cursor rule: `.cursor/rules/rust-best-practices.mdc`

# Rust Best Practices

## Rule
Follow Rust best practices, idioms, and conventions when writing Rust code.

## Code Style
- Use `rustfmt` for consistent formatting (enforced by pre-commit hook)
- Run `clippy` and fix all warnings (enforced by pre-commit hook)
- Prefer `rustfmt` defaults unless project-specific formatting is required
- Use meaningful variable and function names following Rust naming conventions
- Prefer snake_case for functions and variables, PascalCase for types

## Error Handling
- Use `Result<T, E>` for fallible operations, not panics or unwrap() in production code
- Prefer `?` operator for error propagation
- Use `anyhow::Result` or `thiserror` for application-level errors
- Provide meaningful error messages with context
- Use `Option<T>` for nullable values, not `null` or `None` checks
- Avoid `unwrap()` and `expect()` in production code; use proper error handling instead

## Memory Management
- Leverage Rust's ownership system; avoid unnecessary cloning
- Use references (`&T`) when borrowing is sufficient
- Prefer `&str` over `String` for function parameters when ownership isn't needed
- Use `Cow<str>` when you need either owned or borrowed strings
- Consider using `Arc<T>` or `Rc<T>` only when shared ownership is truly needed

## Type Safety
- Use strong types instead of primitive types (e.g., `UserId` instead of `u64`)
- Leverage enums for state machines and discriminated unions
- Use `#[derive]` attributes (Debug, Clone, PartialEq, etc.) when appropriate
- Prefer pattern matching over if-else chains
- Use `match` exhaustively; handle all cases

## Performance
- Avoid premature optimization; write clear, idiomatic code first
- Use `Vec` for dynamic arrays, arrays for fixed-size collections
- Prefer iterators over manual loops when appropriate
- Use `&[T]` slices instead of `Vec<T>` when possible
- Consider using `Box<T>` for large types on the stack
- Profile before optimizing

## Testing
- Write unit tests using `#[cfg(test)]` modules
- Use `#[test]` for test functions
- Prefer `assert_eq!` and `assert!` macros
- Test error cases, not just happy paths
- Use integration tests in `tests/` directory for external API testing
- Consider property-based testing with `proptest` for complex logic

## Documentation
- Write doc comments (`///`) for public APIs
- Use `//!` for module-level documentation
- Include examples in doc comments when helpful
- Document `unsafe` blocks and explain why they're safe
- Use `#[allow(...)]` sparingly and document why

## Dependencies
- Keep dependencies minimal and well-maintained
- Prefer standard library solutions when available
- Use `cargo audit` to check for security vulnerabilities
- Pin dependency versions appropriately in Cargo.toml
- Consider using workspace dependencies for monorepos

## Unsafe Code
- Avoid `unsafe` unless absolutely necessary
- When using `unsafe`, document invariants and safety guarantees
- Prefer safe abstractions over raw unsafe code
- Use `unsafe` blocks, not `unsafe fn`, when possible

## Common Patterns
- Use `match` for exhaustive pattern matching
- Use `if let` and `while let` for Option/Result handling
- Prefer `map`, `and_then`, `or_else` for Option/Result chains
- Use `unwrap_or`, `unwrap_or_else` for default values
- Use `collect()` to convert iterators to collections
- Prefer `Vec::with_capacity()` when size is known

## Project-Specific
- Use `cargo-commitlint` for commit message validation
- Follow Conventional Commits specification
- Run `cargo fmt` and `cargo clippy` before committing
- All tests must pass before pushing (enforced by pre-push hook)

