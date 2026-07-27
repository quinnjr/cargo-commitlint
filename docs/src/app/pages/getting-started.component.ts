import { Component, OnInit } from '@angular/core';
import { RouterLink } from '@angular/router';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-getting-started',
  standalone: true,
  imports: [RouterLink],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">Getting Started</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Installation</h2>
        <div class="text-foreground/80">
          <h3 class="text-xl font-semibold mb-4 text-foreground">From Source</h3>
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-4">
            <pre class="text-sm overflow-x-auto"><code>git clone https://github.com/quinnjr/cargo-commitlint.git
cd cargo-commitlint
cargo install --path .</code></pre>
          </div>

          <h3 class="text-xl font-semibold mb-4 mt-6 text-foreground">From Crates.io</h3>
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm overflow-x-auto"><code>cargo install cargo-commitlint</code></pre>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Install Git Hooks</h2>
        <div class="text-foreground/80">
          <p class="mb-4">After installation, <code class="bg-surface-muted px-2 py-1 rounded">cargo commitlint</code> is available as a cargo subcommand and can install its own git hook.</p>

          <h3 class="text-xl font-semibold mb-4 text-foreground">Built-in Installer</h3>
          <p class="mb-4">Install the commit-msg hook into the current repository:</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-4">
            <pre class="text-sm overflow-x-auto"><code>cargo commitlint install</code></pre>
          </div>
          <p class="mb-4">Pass <code class="bg-surface-muted px-2 py-1 rounded">--force</code> to replace a hook that is already there, and <code class="bg-surface-muted px-2 py-1 rounded">cargo commitlint uninstall</code> to remove it.</p>

          <h3 class="text-xl font-semibold mb-4 mt-6 text-foreground">As a dev-dependency</h3>
          <p class="mb-4">Add it to your Cargo.toml and the build script installs the hook for you:</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-4">
            <pre class="text-sm overflow-x-auto"><code>[dev-dependencies]
cargo-commitlint = "2"

[package.metadata.commitlint]
user-hooks = true</code></pre>
          </div>
          <p class="mb-4"><code class="bg-surface-muted px-2 py-1 rounded">user-hooks = true</code> writes the hook to <code class="bg-surface-muted px-2 py-1 rounded">.commitlint/hooks/</code> so it can be committed and shared. Point git at it once with <code class="bg-surface-muted px-2 py-1 rounded">git config core.hooksPath .commitlint/hooks</code>. Set <code class="bg-surface-muted px-2 py-1 rounded">no-install = true</code> to disable automatic installation, or <code class="bg-surface-muted px-2 py-1 rounded">COMMITLINT_SKIP=1</code> to bypass the hook for one commit.</p>

          <h3 class="text-xl font-semibold mb-4 mt-6 text-foreground">Hook Behavior Without a Binary</h3>
          <p class="mb-4"><strong>The commit-msg hook only validates once a cargo-commitlint binary exists.</strong> The installed hook looks for the binary (in <code class="bg-surface-muted px-2 py-1 rounded">target/release/</code>, <code class="bg-surface-muted px-2 py-1 rounded">target/debug/</code>, then on your <code class="bg-surface-muted px-2 py-1 rounded">PATH</code>). If none is found, the hook prints a warning to stderr and exits successfully, so the commit is accepted <em>without any validation</em>.</p>
          <p class="mb-4">On a fresh clone, build or install it once so the gate is actually active:</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm overflow-x-auto"><code># from the project root
cargo build --release

# or install it globally
cargo install cargo-commitlint</code></pre>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Basic Usage</h2>
        <div class="text-foreground/80">
          <h3 class="text-xl font-semibold mb-4 text-foreground">Validate Commit Messages</h3>
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-4">
            <pre class="text-sm overflow-x-auto"><code># Validate from command line
cargo commitlint check --message "feat: add new feature"

# Validate from stdin
echo "feat: add new feature" | cargo commitlint check</code></pre>
          </div>

          <h3 class="text-xl font-semibold mb-4 mt-6 text-foreground">Configuration</h3>
          <p class="mb-4">Create a <code class="bg-surface-muted px-2 py-1 rounded">commitlint.toml</code> file in your project root:</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm overflow-x-auto"><code>[rules]
[rules.type]
enum = ["feat", "fix", "docs", "style", "refactor", "test", "chore"]
case = "lowercase"

[rules.scope]
enum = []
case = "lowercase"

subject_case = ["sentence-case"]
header_max_length = 72</code></pre>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Next Steps</h2>
        <div class="text-foreground/80">
          <ul class="list-disc list-inside space-y-2">
            <li>Read the <a [routerLink]="['/configuration']" class="text-primary-500 hover:underline">Configuration Guide</a> for detailed options</li>
            <li>Check out <a [routerLink]="['/examples']" class="text-primary-500 hover:underline">Examples</a> for common use cases</li>
            <li>Learn about <a [routerLink]="['/conventional-commits']" class="text-primary-500 hover:underline">Conventional Commits</a> format</li>
          </ul>
        </div>
      </div>
    </div>
  `
})
export class GettingStartedComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'Getting Started - cargo-commitlint',
      description: 'Learn how to install and set up cargo-commitlint in your Rust project. Includes installation instructions, git hook setup, and basic usage examples.',
      keywords: 'cargo-commitlint installation, rust commitlint setup, git hooks rust, conventional commits rust',
      path: 'getting-started'
    });
  }
}

