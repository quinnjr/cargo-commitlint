import { RenderMode, ServerRoute } from '@angular/ssr';

/**
 * Every route is prerendered at build time, so GitHub Pages can serve a real
 * static HTML file per page. Routes are discovered from the client router in
 * `app.routes.ts`; this only declares how they should be rendered.
 */
export const serverRoutes: ServerRoute[] = [
  {
    path: '**',
    renderMode: RenderMode.Prerender
  }
];
