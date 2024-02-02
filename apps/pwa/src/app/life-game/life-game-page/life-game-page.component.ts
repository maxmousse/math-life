import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LifeGameEngineComponent } from '../life-game-engine/life-game-engine.component';

@Component({
  selector: 'ml-life-game-page',
  standalone: true,
  imports: [CommonModule, LifeGameEngineComponent],
  templateUrl: './life-game-page.component.html',
  styleUrl: './life-game-page.component.css',
})
export class LifeGamePageComponent {}
