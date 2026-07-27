import { Component, OnInit } from '@angular/core';
import { RouterLink } from '@angular/router';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [RouterLink],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-6xl">
      <div class="text-center mb-12">
        <h1 class="text-5xl font-bold mb-4 text-foreground">cargo-commitlint</h1>
        <p class="text-xl text-foreground/80 mb-6">
          A Rust-based commit message linter following the Conventional Commits specification
        </p>
        <div class="flex gap-4 justify-center flex-wrap">
          <a [routerLink]="['/getting-started']" class="px-6 py-3 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-medium transition-colors">
            Get Started
          </a>
          <a [routerLink]="['/examples']" class="px-6 py-3 border border-border hover:bg-surface-elevated text-foreground rounded-lg font-medium transition-colors">
            View Examples
          </a>
        </div>
      </div>

      <div class="grid md:grid-cols-3 gap-6 mb-12">
        <div class="bg-surface border border-border rounded-lg p-6">
          <h3 class="text-xl font-semibold mb-3 text-foreground">🚀 Fast & Native</h3>
          <p class="text-foreground/80">Built entirely in Rust for maximum performance. No Node.js runtime required.</p>
        </div>

        <div class="bg-surface border border-border rounded-lg p-6">
          <h3 class="text-xl font-semibold mb-3 text-foreground">⚙️ Configurable</h3>
          <p class="text-foreground/80">Configure via TOML files with sensible defaults. Supports all Conventional Commit types.</p>
        </div>

        <div class="bg-surface border border-border rounded-lg p-6">
          <h3 class="text-xl font-semibold mb-3 text-foreground">🔗 Git Integration</h3>
          <p class="text-foreground/80">Automatic git hook installation via the build script, or install it by hand.</p>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-12">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Features</h2>
        <ul class="list-disc list-inside space-y-2 text-foreground/80">
          <li>✅ Validates commit messages against Conventional Commits specification</li>
          <li>✅ Configurable via TOML (similar to commitlint)</li>
          <li>✅ Git hook installation, with a build script for dev-dependency use</li>
          <li>✅ Supports all standard Conventional Commit types</li>
          <li>✅ Customizable rules for type, scope, subject, body, and footer</li>
          <li>✅ Regex-based commit message parsing</li>
          <li>✅ Ignore patterns for skipping validation</li>
        </ul>
      </div>

      <div class="mt-12 text-center">
        <h2 class="text-3xl font-bold mb-6 text-foreground">Quick Start</h2>
        <div class="bg-surface-elevated border border-border rounded-lg p-6 text-left">
          <pre class="text-foreground overflow-x-auto"><code># Install (from crates.io when published)
cargo install cargo-commitlint

# Or install from source
git clone https://github.com/quinnjr/cargo-commitlint.git
cd cargo-commitlint
cargo install --path .

# Use as a cargo subcommand
cargo commitlint --help

# Install git hook
cargo commitlint install

# Validate a commit message
cargo commitlint check --message "feat: add new feature"</code></pre>
        </div>
      </div>
    </div>
  `
})
export class HomeComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'cargo-commitlint - Rust-based Commit Message Linter',
      description: 'A Rust-based commit message linter following the Conventional Commits specification. Configurable via TOML, JSON or YAML, with a full commitlint rule set and git hook installation for Rust projects.',
      keywords: 'rust, commitlint, conventional commits, git hooks, cargo, rust tooling, commit message validation, code quality, developer tools',
      path: ''
    });
  }
}

