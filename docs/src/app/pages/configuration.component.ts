import { Component, OnInit } from '@angular/core';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-configuration',
  standalone: true,
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">Configuration</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Configuration File</h2>
        <div class="text-foreground/80">
          <p class="mb-4">cargo-commitlint looks for configuration in the following locations (in order):</p>
          <ul class="list-disc list-inside space-y-2 mb-4">
            <li><code class="bg-surface-muted px-2 py-1 rounded">commitlint.toml</code> in the project root</li>
            <li><code class="bg-surface-muted px-2 py-1 rounded">.commitlint.toml</code> in the project root</li>
            <li><code class="bg-surface-muted px-2 py-1 rounded">.cargo/commitlint.toml</code> in the project root</li>
          </ul>
          <p>If no configuration file is found, default settings are used.</p>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Configuration Options</h2>
        <div class="text-foreground/80">
          <div class="mb-6">
            <div class="border-b border-border mb-4">
              <button (click)="activeTab = 'rules'" [class.border-b-2]="activeTab === 'rules'" [class.border-primary-500]="activeTab === 'rules'" class="px-4 py-2 text-foreground">Rules</button>
              <button (click)="activeTab = 'parser'" [class.border-b-2]="activeTab === 'parser'" [class.border-primary-500]="activeTab === 'parser'" class="px-4 py-2 text-foreground">Parser</button>
              <button (click)="activeTab = 'ignores'" [class.border-b-2]="activeTab === 'ignores'" [class.border-primary-500]="activeTab === 'ignores'" class="px-4 py-2 text-foreground">Ignores</button>
            </div>
            @if (activeTab === 'rules') {
            <div>
              <div class="space-y-4">
                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Type Rules</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.type.enum</code> - List of allowed commit types (empty = all allowed)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.type.case</code> - Case requirement (lowercase, uppercase, camel-case, kebab-case, pascal-case, snake-case)</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Scope Rules</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.scope.enum</code> - List of allowed scopes (empty = all allowed)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.scope.case</code> - Case requirement for scope</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Subject Rules</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.subject_case</code> - List of allowed case formats (sentence-case, lowercase, uppercase, start-case)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.subject_empty</code> - Whether subject can be empty</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.subject_full_stop</code> - Character that should not appear at end of subject</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Header Rules</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.header_max_length</code> - Maximum header length (default: 72)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.header_min_length</code> - Minimum header length (default: 0)</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Body & Footer Rules</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.body_leading_blank</code> - Require blank line before body</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.body_max_line_length</code> - Maximum line length in body</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.footer_leading_blank</code> - Require blank line before footer</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.footer_max_line_length</code> - Maximum line length in footer</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Breaking Changes</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">rules.allow_breaking</code> - Whether breaking-change commits (! marker or BREAKING CHANGE footer) are allowed (default: true)</li>
                  </ul>
                </div>
              </div>
            </div>
            }
            @if (activeTab === 'parser') {
            <div>
              <div class="space-y-4">
                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Parser Configuration</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm mb-4">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.pattern</code> - Regex pattern for parsing conventional commits</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.correspondence</code> - Map regex capture groups to commit fields</li>
                  </ul>
                  <div class="bg-surface-elevated border border-border rounded-lg p-4">
                    <pre class="text-sm overflow-x-auto"><code>[parser]
pattern = "^(?P&lt;type&gt;\\w+)(?:\\((?P&lt;scope&gt;[^)]+)\\))?(?P&lt;breaking&gt;!)?:\\s(?P&lt;subject&gt;.*)$"

[parser.correspondence]
type = "type"
scope = "scope"
subject = "subject"
breaking = "breaking"</code></pre>
                  </div>
                </div>
              </div>
            </div>
            }
            @if (activeTab === 'ignores') {
            <div>
              <div class="space-y-4">
                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Ignore Patterns</h3>
                  <p class="mb-4 text-sm">Commits matching these regex patterns will skip validation:</p>
                  <div class="bg-surface-elevated border border-border rounded-lg p-4">
                    <pre class="text-sm overflow-x-auto"><code>ignores = [
    "Merge.*",
    "Revert.*",
]</code></pre>
                  </div>
                </div>
              </div>
            </div>
            }
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Complete Example</h2>
        <div class="text-foreground/80">
          <div class="bg-surface-elevated border border-border rounded-lg p-4">
            <pre class="text-sm overflow-x-auto"><code># Top-level keys must come before any [table] headers
ignores = []

[rules]
subject_case = ["sentence-case"]
subject_empty = false
subject_full_stop = "."

header_max_length = 72
header_min_length = 0

body_leading_blank = true
body_max_line_length = 100

footer_leading_blank = true
footer_max_line_length = 100

allow_breaking = true

[rules.type]
enum = [
    "build", "chore", "ci", "docs", "feat", "fix",
    "perf", "refactor", "revert", "style", "test"
]
case = "lowercase"

[rules.scope]
enum = []
case = "lowercase"

[parser]
pattern = "^(?P&lt;type&gt;\\w+)(?:\\((?P&lt;scope&gt;[^)]+)\\))?(?P&lt;breaking&gt;!)?:\\s(?P&lt;subject&gt;.*)$"

[parser.correspondence]
type = "type"
scope = "scope"
subject = "subject"
breaking = "breaking"</code></pre>
          </div>
        </div>
      </div>
    </div>
  `
})
export class ConfigurationComponent implements OnInit {
  activeTab = 'rules';

  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'Configuration - cargo-commitlint',
      description: 'Complete guide to configuring cargo-commitlint. Learn about TOML configuration options, validation rules, parser settings, and ignore patterns.',
      keywords: 'cargo-commitlint configuration, commitlint.toml, rust commitlint config, conventional commits configuration',
      path: 'configuration'
    });
  }
}
