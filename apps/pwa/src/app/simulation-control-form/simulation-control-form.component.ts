import { Component, EventEmitter, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule, MatLabel } from '@angular/material/form-field';
import { MatButtonToggleModule } from '@angular/material/button-toggle';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatIcon } from '@angular/material/icon';
import { SimulationPlayMode } from '../simulation.types';

@Component({
  selector: 'ml-simulation-control-form',
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
  templateUrl: './simulation-control-form.component.html',
  styleUrl: './simulation-control-form.component.scss',
})
export class SimulationControlFormComponent {
  @Input()
  defaultPlayMode: SimulationPlayMode = 'pause';

  @Output()
  playModeChange = new EventEmitter<SimulationPlayMode>();
  @Output()
  next = new EventEmitter<void>();

  playModeControl = new FormControl<SimulationPlayMode>(this.defaultPlayMode, {
    updateOn: 'change',
  });
}
