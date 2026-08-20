# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

Please report (suspected) security vulnerabilities to **[quinn.josephr@protonmail.com](mailto:quinn.josephr@protonmail.com)**. You will receive a response within 48 hours. If the issue is confirmed, we will release a patch as soon as possible depending on complexity but historically within a few days.

### What to Include

When reporting a security vulnerability, please include:

- **Type of issue** (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- **Full paths of source file(s) related to the manifestation of the issue**
- **The location of the affected source code** (tag/branch/commit or direct URL)
- **Step-by-step instructions to reproduce the issue**
- **Proof-of-concept or exploit code** (if possible)
- **Impact of the issue**, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

### Disclosure Policy

- We will acknowledge receipt of your vulnerability report within 48 hours
- We will send you regular updates about our progress
- We will notify you when the vulnerability has been fixed
- We will credit you for the discovery (unless you prefer to remain anonymous)

### What to Expect

- **48 hours**: Initial response to your report
- **7 days**: Status update on the vulnerability
- **30 days**: Target fix release date (may vary based on severity)

## Security Best Practices

When using cargo-commitlint:

1. **Keep dependencies updated**: Regularly update your Rust toolchain and dependencies
2. **Review configuration**: Ensure your `commitlint.toml` doesn't expose sensitive information
3. **Use git hooks**: Enable git hooks to catch issues early
4. **Audit dependencies**: Run `cargo audit` regularly to check for known vulnerabilities

## Security Updates

Security updates will be announced via:
- GitHub Security Advisories
- Release notes
- Project documentation

## Credits

We appreciate responsible disclosure of security vulnerabilities. Contributors who report valid security issues will be credited in our security advisories (unless they prefer to remain anonymous).

