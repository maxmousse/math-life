import { Component, EventEmitter, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule, MatLabel } from '@angular/material/form-field';
import { MatButtonToggleModule } from '@angular/material/button-toggle';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { LifeGamePlayMode } from '../life-game.types';
import { MatIcon } from '@angular/material/icon';

@Component({
  selector: 'ml-life-game-form',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MatFormFieldModule,
    MatButtonToggleModule,
    MatButtonModule,
    MatInputModule,
    MatFormFieldModule,
    MatLabel,
    MatIcon,
  ],
  templateUrl: './life-game-form.component.html',
  styleUrl: './life-game-form.component.css',
})
export class LifeGameFormComponent {
  @Input()
  defaultPlayMode: LifeGamePlayMode = 'pause';
  @Input()
  defaultSpeed = 100;

  @Output()
  playModeChange = new EventEmitter<LifeGamePlayMode>();
  @Output()
  speedChange = new EventEmitter();
  @Output()
  next = new EventEmitter<void>();

  playModeControl = new FormControl<LifeGamePlayMode>(this.defaultPlayMode, {
    updateOn: 'change',
  });
  speedControl = new FormControl(this.defaultSpeed);
}
