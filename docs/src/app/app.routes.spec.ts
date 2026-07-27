import { Type } from '@angular/core';
import { TestBed } from '@angular/core/testing';
import { Router, provideRouter } from '@angular/router';
import { RouterTestingHarness } from '@angular/router/testing';

import { routes } from './app.routes';
import { ApiReferenceComponent } from './pages/api-reference.component';
import { ConfigurationComponent } from './pages/configuration.component';
import { ContributingComponent } from './pages/contributing.component';
import { ConventionalCommitsComponent } from './pages/conventional-commits.component';
import { ExamplesComponent } from './pages/examples.component';
import { GettingStartedComponent } from './pages/getting-started.component';
import { HomeComponent } from './pages/home.component';

/**
 * Every navigable route, paired with the component that must be activated.
 *
 * Six of the seven routes resolve their component through a lazy
 * `loadComponent: () => import(...)`. A typo'd module path or a renamed exported
 * symbol still type-checks and still builds -- it only blows up at runtime, when
 * a user navigates. These cases force each dynamic import to actually run.
 */
const routeCases: ReadonlyArray<[url: string, component: Type<unknown>]> = [
  ['/', HomeComponent],
  ['/getting-started', GettingStartedComponent],
  ['/configuration', ConfigurationComponent],
  ['/examples', ExamplesComponent],
  ['/conventional-commits', ConventionalCommitsComponent],
  ['/api-reference', ApiReferenceComponent],
  ['/contributing', ContributingComponent],
];

describe('app routes', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideRouter(routes)],
    });
  });

  it.each(routeCases)('resolves %s to its page component', async (url, component) => {
    const harness = await RouterTestingHarness.create();
    const activated = await harness.navigateByUrl(url, component);

    expect(activated).toBeInstanceOf(component);
    expect(TestBed.inject(Router).url).toBe(url);
  });

  it('redirects an unknown path to the home route', async () => {
    const harness = await RouterTestingHarness.create();
    const activated = await harness.navigateByUrl('/no-such-page', HomeComponent);

    expect(activated).toBeInstanceOf(HomeComponent);
    expect(TestBed.inject(Router).url).toBe('/');
  });

  it('covers every declared route', () => {
    const declared = routes
      .map(route => route.path)
      .filter((path): path is string => path !== undefined && path !== '**')
      .map(path => `/${path}`);
    const covered = routeCases.map(([url]) => url);

    expect(covered.slice().sort()).toEqual(declared.slice().sort());
  });
});
