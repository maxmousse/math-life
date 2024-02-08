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
  selector: 'ml-life-game-control-form',
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
  templateUrl: './life-game-control-form.component.html',
  styleUrl: './life-game-control-form.component.css',
})
export class LifeGameControlFormComponent {
  @Input()
  defaultPlayMode: LifeGamePlayMode = 'pause';

  @Output()
  playModeChange = new EventEmitter<LifeGamePlayMode>();
  @Output()
  next = new EventEmitter<void>();

  playModeControl = new FormControl<LifeGamePlayMode>(this.defaultPlayMode, {
    updateOn: 'change',
  });
}
