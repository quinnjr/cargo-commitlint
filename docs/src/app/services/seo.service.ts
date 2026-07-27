import { Injectable } from '@angular/core';
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
    const url =
      data.canonicalUrl ||
      data.ogUrl ||
      (data.path ? `${this.baseUrl}/${data.path}` : `${this.baseUrl}/`);

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
    let link: HTMLLinkElement | null = document.querySelector('link[rel="canonical"]');
    if (!link) {
      link = document.createElement('link');
      link.setAttribute('rel', 'canonical');
      document.head.appendChild(link);
    }
    link.setAttribute('href', url);
  }
}

