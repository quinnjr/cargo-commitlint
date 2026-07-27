import { Routes } from '@angular/router';
import { HomeComponent } from './pages/home.component';

export const routes: Routes = [
  { path: '', component: HomeComponent },
  { path: 'getting-started', loadComponent: () => import('./pages/getting-started.component').then(m => m.GettingStartedComponent) },
  { path: 'configuration', loadComponent: () => import('./pages/configuration.component').then(m => m.ConfigurationComponent) },
  { path: 'examples', loadComponent: () => import('./pages/examples.component').then(m => m.ExamplesComponent) },
  { path: 'conventional-commits', loadComponent: () => import('./pages/conventional-commits.component').then(m => m.ConventionalCommitsComponent) },
  { path: 'api-reference', loadComponent: () => import('./pages/api-reference.component').then(m => m.ApiReferenceComponent) },
  { path: 'contributing', loadComponent: () => import('./pages/contributing.component').then(m => m.ContributingComponent) },
  { path: '**', redirectTo: '' }
];
