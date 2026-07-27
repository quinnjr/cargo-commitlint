import { DOCUMENT, Injectable, inject } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';

export interface SEOData {
  title?: string;
  description?: string;
  keywords?: string;
  ogTitle?: string;
  ogImage?: string;
  ogUrl?: string;
  twitterTitle?: string;
  /**
   * Route path relative to the site root, e.g. `'configuration'`; omit or pass
   * `''` for the home page. The absolute canonical URL is composed from
   * `SEOService`'s `baseUrl` so the host lives in exactly one place.
   */
  path?: string;
  /** Explicit absolute canonical URL. Overrides `path` when both are given. */
  canonicalUrl?: string;
}

@Injectable({
  providedIn: 'root'
})
export class SEOService {
  private baseUrl = 'https://quinnjr.github.io/cargo-commitlint';
  private defaultTitle = 'cargo-commitlint - Rust-based Commit Message Linter';
  private defaultDescription = 'A Rust-based commit message linter following the Conventional Commits specification. Configurable via TOML, integrates with cargo-husky, and validates commit messages for Rust projects.';
  private defaultKeywords = 'rust, commitlint, conventional commits, git hooks, cargo, rust tooling, commit message validation, code quality, developer tools';

  // Injected rather than using the global `document`, which does not exist
  // while the site is being prerendered on the server.
  private readonly doc = inject(DOCUMENT);

  constructor(
    private title: Title,
    private meta: Meta
  ) {}

  updateSEO(data: SEOData): void {
    // Update title
    const title = data.title || this.defaultTitle;
    this.title.setTitle(title);
    this.meta.updateTag({ name: 'title', content: title });

    // Update description
    const description = data.description || this.defaultDescription;
    this.meta.updateTag({ name: 'description', content: description });
    this.meta.updateTag({ property: 'og:description', content: description });
    this.meta.updateTag({ property: 'twitter:description', content: description });
    this.meta.updateTag({ name: 'ai:description', content: description });

    // Update keywords
    const keywords = data.keywords || this.defaultKeywords;
    this.meta.updateTag({ name: 'keywords', content: keywords });
    this.meta.updateTag({ name: 'ai:keywords', content: keywords });

    // Resolve the canonical URL: an explicit `canonicalUrl` wins, then `ogUrl`,
    // otherwise it is composed from `baseUrl` and the route-relative `path`
    // (the home page keeps its trailing slash, sub-pages have none).
    // Trailing slashes match what GitHub Pages actually serves: each route is
    // prerendered to `<path>/index.html`, so `/configuration` 301-redirects to
    // `/configuration/`. Pointing the canonical at the redirect target avoids
    // advertising a URL that never returns 200.
    const url =
      data.canonicalUrl ||
      data.ogUrl ||
      (data.path ? `${this.baseUrl}/${data.path}/` : `${this.baseUrl}/`);

    // Update Open Graph
    this.meta.updateTag({ property: 'og:title', content: data.ogTitle || title });
    this.meta.updateTag({ property: 'og:url', content: url });
    if (data.ogImage) {
      this.meta.updateTag({ property: 'og:image', content: data.ogImage });
    }

    // Update Twitter
    this.meta.updateTag({ property: 'twitter:title', content: data.twitterTitle || title });
    this.meta.updateTag({ property: 'twitter:url', content: url });

    // Update canonical URL
    let link: HTMLLinkElement | null = this.doc.querySelector('link[rel="canonical"]');
    if (!link) {
      link = this.doc.createElement('link');
      link.setAttribute('rel', 'canonical');
      this.doc.head.appendChild(link);
    }
    link.setAttribute('href', url);
  }
}

