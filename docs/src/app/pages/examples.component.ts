import { Component, OnInit } from '@angular/core';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-examples',
  standalone: true,
  imports: [],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">Examples</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Valid Commit Messages</h2>
        <div class="text-foreground/80">
          <div class="space-y-4">
            <div>
              <span class="inline-block bg-success-500 text-white px-2 py-1 rounded text-xs mb-2">Valid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">feat: add user authentication</code>
              </div>
            </div>

            <div>
              <span class="inline-block bg-success-500 text-white px-2 py-1 rounded text-xs mb-2">Valid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">feat(api): add new endpoint</code>
              </div>
            </div>

            <div>
              <span class="inline-block bg-success-500 text-white px-2 py-1 rounded text-xs mb-2">Valid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">fix: resolve memory leak in parser</code>
              </div>
            </div>

            <div>
              <span class="inline-block bg-success-500 text-white px-2 py-1 rounded text-xs mb-2">Valid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">docs: update README with installation instructions</code>
              </div>
            </div>

            <div>
              <span class="inline-block bg-warning-500 text-white px-2 py-1 rounded text-xs mb-2">Breaking Change</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">feat!: breaking change in API</code>
              </div>
            </div>

            <div>
              <span class="inline-block bg-success-500 text-white px-2 py-1 rounded text-xs mb-2">With Body</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>feat: add feature

This is a longer description of the change.

Closes #123</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Invalid Commit Messages</h2>
        <div class="text-foreground/80">
          <div class="space-y-4">
            <div>
              <span class="inline-block bg-error-500 text-white px-2 py-1 rounded text-xs mb-2">Invalid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">invalid: bad commit type</code>
              </div>
              <p class="text-sm text-foreground/60 mt-2">Error: Type 'invalid' is not in the allowed enum</p>
            </div>

            <div>
              <span class="inline-block bg-error-500 text-white px-2 py-1 rounded text-xs mb-2">Invalid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">feat:Missing space after colon</code>
              </div>
              <p class="text-sm text-foreground/60 mt-2">Error: Missing space after colon</p>
            </div>

            <div>
              <span class="inline-block bg-error-500 text-white px-2 py-1 rounded text-xs mb-2">Invalid</span>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <code class="text-sm">feat: subject too long and exceeds the maximum length limit of 72 characters</code>
              </div>
              <p class="text-sm text-foreground/60 mt-2">Error: Header exceeds maximum length of 72 characters</p>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Conventional Commit Types</h2>
        <div class="text-foreground/80">
          <div class="grid md:grid-cols-2 gap-4">
            <div>
              <h3 class="font-semibold mb-2 text-foreground">Common Types</h3>
              <ul class="space-y-1 text-sm">
                <li><code class="bg-surface-muted px-2 py-1 rounded">feat</code> - A new feature</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">fix</code> - A bug fix</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">docs</code> - Documentation changes</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">style</code> - Code style changes</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">refactor</code> - Code refactoring</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">test</code> - Test additions/changes</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">chore</code> - Maintenance tasks</li>
              </ul>
            </div>
            <div>
              <h3 class="font-semibold mb-2 text-foreground">Additional Types</h3>
              <ul class="space-y-1 text-sm">
                <li><code class="bg-surface-muted px-2 py-1 rounded">perf</code> - Performance improvements</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">build</code> - Build system changes</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">ci</code> - CI configuration changes</li>
                <li><code class="bg-surface-muted px-2 py-1 rounded">revert</code> - Reverts a previous commit</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">CLI Usage Examples</h2>
        <div class="text-foreground/80">
          <div class="space-y-4">
            <div>
              <h3 class="font-semibold mb-2 text-foreground">Install Hook</h3>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint install</code></pre>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">Validate Message</h3>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint check --message "feat: add feature"</code></pre>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">Validate from Stdin</h3>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>echo "feat: add feature" | cargo commitlint check</code></pre>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">Custom Config</h3>
              <div class="bg-surface-elevated border border-border rounded-lg p-4">
                <pre class="text-sm"><code>cargo commitlint check --message "feat: add feature" --config ./custom.toml</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
})
export class ExamplesComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'Examples - cargo-commitlint',
      description: 'Examples of valid and invalid commit messages for cargo-commitlint. Learn how to write proper Conventional Commits with real-world examples.',
      keywords: 'cargo-commitlint examples, conventional commits examples, commit message examples, rust commitlint examples',
      path: 'examples'
    });
  }
}

