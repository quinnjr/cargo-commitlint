import { Component, OnInit } from '@angular/core';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-contributing',
  standalone: true,
  imports: [],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">Contributing</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Getting Started</h2>
        <div class="text-foreground/80">
          <p class="mb-4">Contributions are welcome! Here's how to get started:</p>
          <ol class="list-decimal list-inside space-y-2 text-sm">
            <li>Fork the repository</li>
            <li>Clone your fork: <code class="bg-surface-muted px-2 py-1 rounded">git clone https://github.com/your-username/cargo-commitlint.git</code></li>
            <li>Create a feature branch: <code class="bg-surface-muted px-2 py-1 rounded">git checkout -b feature/your-feature</code></li>
            <li>Make your changes</li>
            <li>Run tests: <code class="bg-surface-muted px-2 py-1 rounded">cargo test</code></li>
            <li>Ensure code formatting: <code class="bg-surface-muted px-2 py-1 rounded">cargo fmt</code></li>
            <li>Check with clippy: <code class="bg-surface-muted px-2 py-1 rounded">cargo clippy</code></li>
            <li>Commit your changes (following Conventional Commits!)</li>
            <li>Push to your fork and create a Pull Request</li>
          </ol>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Development Setup</h2>
        <div class="text-foreground/80">
          <div class="space-y-4">
            <div>
              <h3 class="font-semibold mb-2 text-foreground">Prerequisites</h3>
              <ul class="list-disc list-inside space-y-1 text-sm">
                <li>Rust (latest stable version)</li>
                <li>Cargo</li>
                <li>Git</li>
              </ul>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">Building</h3>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code># Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Git Hooks</h2>
        <div class="text-foreground/80">
          <p class="mb-4 text-sm">This project uses cargo-husky for git hooks. The hooks are automatically installed when you run <code class="bg-surface-muted px-2 py-1 rounded">cargo test</code>.</p>
          <p class="mb-4 text-sm">Available hooks:</p>
          <ul class="list-disc list-inside space-y-1 text-sm">
            <li><strong>pre-commit</strong>: Runs cargo fmt --check and cargo clippy</li>
            <li><strong>pre-push</strong>: Runs cargo test</li>
            <li><strong>commit-msg</strong>: Validates commit messages using cargo-commitlint</li>
          </ul>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Commit Messages</h2>
        <div class="text-foreground/80">
          <p class="mb-4 text-sm">Please follow the Conventional Commits specification for all commit messages. This project uses cargo-commitlint to enforce this!</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm"><code>feat: add new feature
fix: resolve bug
docs: update documentation
refactor: improve code structure</code></pre>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Pull Request Process</h2>
        <div class="text-foreground/80">
          <ol class="list-decimal list-inside space-y-2 text-sm">
            <li>Ensure your code follows Rust best practices</li>
            <li>Add tests for new functionality</li>
            <li>Update documentation if needed</li>
            <li>Ensure all tests pass</li>
            <li>Ensure code is properly formatted</li>
            <li>Create a descriptive Pull Request</li>
            <li>Wait for review and address any feedback</li>
          </ol>
        </div>
      </div>
    </div>
  `
})
export class ContributingComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'Contributing - cargo-commitlint',
      description: 'Learn how to contribute to cargo-commitlint. Includes development setup, git hooks, commit message guidelines, and pull request process.',
      keywords: 'contribute to cargo-commitlint, rust open source, cargo-commitlint development, pull request guidelines',
      path: 'contributing'
    });
  }
}

