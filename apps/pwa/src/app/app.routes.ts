import { Route } from '@angular/router';
import { LifeGamePageComponent } from './life-game/life-game-page/life-game-page.component';

export const appRoutes: Route[] = [
  {
    path: '**',
    component: LifeGamePageComponent,
  },
];
