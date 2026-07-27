import { BootstrapContext, bootstrapApplication } from '@angular/platform-browser';

import { App } from './app/app';
import { config } from './app/app.config.server';

/**
 * Entry point used by the prerenderer to render each route to static HTML.
 *
 * The `context` argument is required when bootstrapping on the server -- without
 * it there is no platform to render into and route extraction fails with NG0401.
 */
export default (context: BootstrapContext) => bootstrapApplication(App, config, context);
