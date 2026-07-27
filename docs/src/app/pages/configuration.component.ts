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
            <li><code class="bg-surface-muted px-2 py-1 rounded">commitlint.toml</code> or <code class="bg-surface-muted px-2 py-1 rounded">.commitlint.toml</code></li>
            <li><code class="bg-surface-muted px-2 py-1 rounded">.commitlintrc.toml</code>, <code class="bg-surface-muted px-2 py-1 rounded">.commitlintrc.json</code>, <code class="bg-surface-muted px-2 py-1 rounded">.commitlintrc.yaml</code> or <code class="bg-surface-muted px-2 py-1 rounded">.commitlintrc.yml</code></li>
            <li><code class="bg-surface-muted px-2 py-1 rounded">.cargo/commitlint.toml</code></li>
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
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Rule Shape</h3>
                  <p class="mb-2 text-sm">Every rule is configured the same way:</p>
                  <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-2">
                    <pre class="text-sm overflow-x-auto"><code>[rules.type-enum]
level = 2             # 0 = disabled, 1 = warning, 2 = error
applicable = "always" # or "never", which inverts the rule
value = ["feat", "fix"]</code></pre>
                  </div>
                  <p class="text-sm"><code class="bg-surface-muted px-2 py-1 rounded">applicable = "never"</code> inverts the check, so <code class="bg-surface-muted px-2 py-1 rounded">type-empty</code> with <code class="bg-surface-muted px-2 py-1 rounded">never</code> means the type must not be empty. Omit <code class="bg-surface-muted px-2 py-1 rounded">value</code> when a rule takes none.</p>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Type &amp; Scope</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">type-enum</code> - Allowed commit types</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">type-case</code> - Case requirement for the type</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">type-empty</code> - Whether the type may be empty</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">type-max-length / type-min-length</code> - Length bounds for the type</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">scope-enum</code> - Allowed scopes</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">scope-case</code> - Case requirement for the scope</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">scope-empty</code> - Whether the scope may be empty</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">scope-max-length / scope-min-length</code> - Length bounds for the scope</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Subject &amp; Header</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">subject-case</code> - Allowed case formats for the subject</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">subject-empty</code> - Whether the subject may be empty</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">subject-full-stop</code> - Trailing punctuation on the subject</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">subject-exclamation-mark</code> - The breaking-change ! marker</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">subject-max-length / subject-min-length</code> - Length bounds for the subject</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">header-max-length</code> - Maximum header length (default: 100)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">header-min-length</code> - Minimum header length</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">header-case / header-full-stop / header-trim</code> - Header case, punctuation and whitespace</li>
                  </ul>
                </div>

                <div>
                  <h3 class="text-lg font-semibold mb-2 text-foreground">Body, Footer &amp; Other</h3>
                  <ul class="list-disc list-inside space-y-1 text-sm">
                    <li><code class="bg-surface-muted px-2 py-1 rounded">body-leading-blank</code> - Require a blank line before the body</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">body-max-line-length</code> - Maximum line length in the body (default: 100)</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">body-case / body-empty / body-full-stop</code> - Body case, presence and punctuation</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">body-max-length / body-min-length</code> - Length bounds for the body</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">footer-leading-blank</code> - Require a blank line before the footer</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">footer-max-line-length</code> - Maximum line length in the footer</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">footer-empty / footer-max-length / footer-min-length</code> - Footer presence and length bounds</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">references-empty / signed-off-by / trailer-exists</code> - Issue references and required trailers</li>
                  </ul>
                  <p class="mt-2 text-sm">Run <code class="bg-surface-muted px-2 py-1 rounded">cargo commitlint print-config</code> to see all 36 rules with their resolved defaults.</p>
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
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.header_pattern</code> - Regex used to split the commit header</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.header_correspondence</code> - Names of the pattern&apos;s capture groups</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.note_keywords</code> - Breaking-change trailers</li>
                    <li><code class="bg-surface-muted px-2 py-1 rounded">parser.reference_actions</code> - Issue-closing keywords such as closes and fixes</li>
                  </ul>
                  <div class="bg-surface-elevated border border-border rounded-lg p-4">
                    <pre class="text-sm overflow-x-auto"><code>[parser]
header_pattern = "^(?P&lt;type&gt;\\w+)(?:\\((?P&lt;scope&gt;[^)]+)\\))?(?P&lt;breaking&gt;!)?:\\s*(?P&lt;subject&gt;.*)$"
header_correspondence = ["type", "scope", "subject"]
note_keywords = ["BREAKING CHANGE", "BREAKING-CHANGE"]</code></pre>
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
            <pre class="text-sm overflow-x-auto"><code># Skip validation for commits matching these regex patterns
ignores = ["^Merge branch", "^Revert "]

# Built-in ignores for merge, revert and squash commits
defaultIgnores = true

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

# level 1 warns without failing the commit
[rules.body-leading-blank]
level = 1
applicable = "always"

[parser]
header_pattern = "^(?P&lt;type&gt;\\w+)(?:\\((?P&lt;scope&gt;[^)]+)\\))?(?P&lt;breaking&gt;!)?:\\s*(?P&lt;subject&gt;.*)$"</code></pre>
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
