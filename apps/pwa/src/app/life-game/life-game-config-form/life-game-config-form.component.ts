import { Component, EventEmitter, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatSelectModule } from '@angular/material/select';
import { LifeGameConfig } from '../life-game.types';
import { lifeGamePatterns } from '../life-game-patterns';

@Component({
  selector: 'ml-life-game-config-form',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatSelectModule,
  ],
  templateUrl: './life-game-config-form.component.html',
  styleUrl: './life-game-config-form.component.css',
})
export class LifeGameConfigFormComponent {
  @Input()
  defaultWidth = 100;
  @Input()
  defaultHeight = 100;

  @Output()
  update = new EventEmitter<LifeGameConfig>();

  patterns = lifeGamePatterns;

  form = new FormGroup({
    width: new FormControl(this.defaultWidth, {
      nonNullable: true,
    }),
    height: new FormControl(this.defaultHeight, {
      nonNullable: true,
    }),
    pattern: new FormControl<string>('', { nonNullable: true }),
  });

  submit() {
    // config type is overwritten to remove partial as
    // form controls will never be disabled, so never be undefined
    // (see here: https://angular.io/guide/typed-forms#partial-values )
    this.update.emit(this.form.value as LifeGameConfig);
  }
}
