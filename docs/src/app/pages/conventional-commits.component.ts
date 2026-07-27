import { Component, OnInit } from '@angular/core';
import { SEOService } from '../services/seo.service';

@Component({
  selector: 'app-conventional-commits',
  standalone: true,
  imports: [],
  template: `
    <div class="container mx-auto px-4 py-12 max-w-4xl">
      <h1 class="text-4xl font-bold mb-8 text-foreground">Conventional Commits</h1>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">What are Conventional Commits?</h2>
        <div class="text-foreground/80">
          <p class="mb-4">
            The Conventional Commits specification is a lightweight convention on top of commit messages.
            It provides an easy set of rules for creating an explicit commit history, which makes it easier
            to write automated tools on top of.
          </p>
          <p>
            cargo-commitlint enforces this specification to ensure consistent, meaningful commit messages
            across your project.
          </p>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Format</h2>
        <div class="text-foreground/80">
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mb-4">
            <pre class="text-sm"><code>&lt;type&gt;[optional scope]: &lt;description&gt;

[optional body]

[optional footer(s)]</code></pre>
          </div>

          <h3 class="text-lg font-semibold mb-2 text-foreground">Components</h3>
          <ul class="list-disc list-inside space-y-2 text-sm">
            <li><strong>type</strong>: The type of change (feat, fix, docs, etc.)</li>
            <li><strong>scope</strong>: Optional scope indicating what part of the codebase is affected</li>
            <li><strong>description</strong>: A short description of the change</li>
            <li><strong>body</strong>: Optional longer description</li>
            <li><strong>footer</strong>: Optional footer for breaking changes or issue references</li>
          </ul>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6 mb-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Commit Types</h2>
        <div class="text-foreground/80">
          <div class="space-y-4">
            <div>
              <h3 class="font-semibold mb-2 text-foreground">feat</h3>
              <p class="text-sm">A new feature. Introduces new functionality to the codebase.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">feat: add user authentication</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">fix</h3>
              <p class="text-sm">A bug fix. Fixes a problem in existing functionality.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">fix: resolve memory leak in parser</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">docs</h3>
              <p class="text-sm">Documentation only changes. No code changes.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">docs: update API documentation</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">style</h3>
              <p class="text-sm">Changes that do not affect the meaning of the code (formatting, missing semicolons, etc.)</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">style: format code with rustfmt</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">refactor</h3>
              <p class="text-sm">A code change that neither fixes a bug nor adds a feature.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">refactor: simplify error handling</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">perf</h3>
              <p class="text-sm">A code change that improves performance.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">perf: optimize database queries</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">test</h3>
              <p class="text-sm">Adding missing tests or correcting existing tests.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">test: add unit tests for parser</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">build</h3>
              <p class="text-sm">Changes that affect the build system or external dependencies.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">build: update Cargo.toml dependencies</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">ci</h3>
              <p class="text-sm">Changes to CI configuration files and scripts.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">ci: add GitHub Actions workflow</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">chore</h3>
              <p class="text-sm">Other changes that don't modify src or test files.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">chore: update .gitignore</code>
              </div>
            </div>

            <div>
              <h3 class="font-semibold mb-2 text-foreground">revert</h3>
              <p class="text-sm">Reverts a previous commit.</p>
              <div class="bg-surface-elevated border border-border rounded-lg p-2 mt-2">
                <code class="text-xs">revert: revert "feat: add feature"</code>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-foreground">Breaking Changes</h2>
        <div class="text-foreground/80">
          <p class="mb-4">
            Breaking changes can be indicated in two ways:
          </p>
          <ol class="list-decimal list-inside space-y-2 text-sm">
            <li>Add a <code class="bg-surface-muted px-2 py-1 rounded">!</code> after the type/scope: <code class="bg-surface-muted px-2 py-1 rounded">feat!: breaking change</code></li>
            <li>Add <code class="bg-surface-muted px-2 py-1 rounded">BREAKING CHANGE:</code> in the footer</li>
          </ol>
          <div class="bg-surface-elevated border border-border rounded-lg p-4 mt-4">
            <pre class="text-sm"><code>feat!: remove deprecated API

BREAKING CHANGE: The old API has been removed.
Use the new API instead.</code></pre>
          </div>
        </div>
      </div>
    </div>
  `
})
export class ConventionalCommitsComponent implements OnInit {
  constructor(private seo: SEOService) {}

  ngOnInit(): void {
    this.seo.updateSEO({
      title: 'Conventional Commits - cargo-commitlint',
      description: 'Learn about the Conventional Commits specification and how cargo-commitlint enforces it. Understand commit types, formats, and breaking changes.',
      keywords: 'conventional commits, commit message format, conventional commits specification, commit types, breaking changes',
      path: 'conventional-commits'
    });
  }
}

