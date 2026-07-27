import { ApplicationConfig, mergeApplicationConfig } from '@angular/core';
import { provideServerRendering, withRoutes } from '@angular/ssr';

import { appConfig } from './app.config';
import { serverRoutes } from './app.routes.server';

/**
 * Server-side configuration used when the site is prerendered.
 *
 * The build emits one static HTML file per route, so each page ships with the
 * title, description and canonical URL that `SEOService` sets in `ngOnInit`
 * already present in the markup, rather than only after the bundle executes.
 */
const serverConfig: ApplicationConfig = {
  providers: [provideServerRendering(withRoutes(serverRoutes))]
};

export const config = mergeApplicationConfig(appConfig, serverConfig);
