import { Component, OnInit } from '@angular/core';
import { RouterLink } from '@angular/router';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-api-reference',
  standalone: true,
  imports: [RouterLink],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">API Reference</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">CLI Commands</h2>
        <div class="text-foreground/80">
          <div class="space-y-6">
            <div>
              <h3 class="text-lg font-semibold mb-2 text-foreground">install</h3>
              <p class="text-sm mb-2">Install git commit-msg hook for commit message validation.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint install</code></pre>
              </div>
              <p class="text-sm mt-2">Installs a git hook that automatically validates commit messages using <code class="bg-surface-muted px-2 py-1 rounded">cargo commitlint</code>.</p>
            </div>

            <div>
              <h3 class="text-lg font-semibold mb-2 text-foreground">uninstall</h3>
              <p class="text-sm mb-2">Remove git commit-msg hook.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint uninstall</code></pre>
              </div>
              <p class="text-sm mt-2">Removes the commit-msg hook installed by <code class="bg-surface-muted px-2 py-1 rounded">cargo commitlint install</code>.</p>
            </div>

            <div>
              <h3 class="text-lg font-semibold mb-2 text-foreground">check</h3>
              <p class="text-sm mb-2">Validate a commit message.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint check [OPTIONS]

Options:
  -m, --message &lt;MESSAGE&gt;    Commit message to validate
  -c, --config &lt;CONFIG&gt;      Path to configuration file
  -h, --help                 Print help</code></pre>
              </div>
              <p class="text-sm mt-2">If <code class="bg-surface-muted px-2 py-1 rounded">--message</code> is not provided, the commit message will be read from stdin. This is useful for git hooks.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-4 mt-2">
                <pre class="text-sm"><code># Examples:
cargo commitlint check --message "feat: add feature"
echo "fix: bug" | cargo commitlint check
cargo commitlint check --message "docs: update" --config ./custom.toml</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Exit Codes</h2>
        <div class="text-foreground/80">
          <ul class="space-y-2 text-sm">
            <li><code class="bg-surface-muted px-2 py-1 rounded">0</code> - Success (commit message is valid)</li>
            <li><code class="bg-surface-muted px-2 py-1 rounded">1</code> - Error (commit message validation failed or other error)</li>
          </ul>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Configuration File Format</h2>
        <div class="text-foreground/80">
          <p class="mb-4 text-sm">Configuration files use TOML format. See the <a routerLink="/configuration" class="text-primary-500 hover:underline">Configuration</a> page for detailed options.</p>
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm overflow-x-auto"><code>[rules]
[rules.type]
enum = ["feat", "fix", "docs"]
case = "lowercase"

[rules.scope]
enum = []
case = "lowercase"

subject_case = ["sentence-case"]
header_max_length = 72

[parser]
pattern = "^(?P&lt;type&gt;\\w+)(?:\\((?P&lt;scope&gt;[^)]+)\\))?(?P&lt;breaking&gt;!)?:\\s(?P&lt;subject&gt;.*)$"

ignores = []</code></pre>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Environment Variables</h2>
        <div class="text-foreground/80">
          <p class="text-sm mb-4">Currently, cargo-commitlint does not use any environment variables. All configuration is done via TOML files or command-line arguments.</p>
        </div>
      </div>
    </div>
  `
})
export class ApiReferenceComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'API Reference - cargo-commitlint',
      description: 'Complete API reference for cargo-commitlint CLI commands, configuration options, exit codes, and environment variables.',
      keywords: 'cargo-commitlint API, commitlint CLI, rust commitlint commands, cargo-commitlint reference',
      path: 'api-reference'
    });
  }
}

