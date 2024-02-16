import { Route } from '@angular/router';
import { HomeComponent } from './home/home.component';

export const appRoutes: Route[] = [
  {
    path: 'home',
    component: HomeComponent,
  },
  {
    path: 'lenia',
    loadComponent: () =>
      import('./lenia/lenia.component').then((mod) => mod.LeniaComponent),
  },
  {
    path: 'conway-game-of-life',
    loadComponent: () =>
      import('./life-game/life-game-page/life-game-page.component').then(
        (mod) => mod.LifeGamePageComponent
      ),
  },
  {
    path: '**',
    pathMatch: 'full',
    redirectTo: 'home',
  },
];
