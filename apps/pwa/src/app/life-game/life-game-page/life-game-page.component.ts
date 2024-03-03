import { ChangeDetectorRef, Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LifeGameEngineComponent } from '../life-game-engine/life-game-engine.component';
import { LifeGameConfigFormComponent } from '../life-game-config-form/life-game-config-form.component';
import { Subject, from, merge, of, switchMap, tap } from 'rxjs';
import { LifeGameConfig } from '../life-game.types';
import { MatTabsModule } from '@angular/material/tabs';

@Component({
  selector: 'ml-life-game-page',
  standalone: true,
  imports: [
    CommonModule,
    LifeGameEngineComponent,
    LifeGameConfigFormComponent,
    MatTabsModule,
  ],
  templateUrl: './life-game-page.component.html',
  styleUrl: './life-game-page.component.scss',
})
export class LifeGamePageComponent {
  gameConfigUpdateSubject = new Subject<LifeGameConfig>();

  // Game config state
  // On game config update, the state is transitively set
  // to `null` to blink the game engine to a fresh state
  gameConfig$ = merge(
    of({ width: 100, height: 100, pattern: '' }),
    this.gameConfigUpdateSubject
      .asObservable()
      .pipe(switchMap((config) => from([null, config])))
  ).pipe(tap(() => this.ref.detectChanges()));

  constructor(private ref: ChangeDetectorRef) {}
}
