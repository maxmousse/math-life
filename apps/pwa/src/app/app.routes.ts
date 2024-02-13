import { Route } from '@angular/router';
import { LifeGamePageComponent } from './life-game/life-game-page/life-game-page.component';
import { LeniaComponent } from './lenia/lenia.component';

export const appRoutes: Route[] = [
  {
    path: 'lenia',
    component: LeniaComponent,
  },
  {
    path: '**',
    component: LifeGamePageComponent,
  },
];
