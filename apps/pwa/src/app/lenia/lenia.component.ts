import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { lenia } from '@ml/lenia';
import { HeatmapComponent } from '../heatmap/heatmap.component';
import { memory } from '@ml/lenia/lenia_bg.wasm';
import { MatTabsModule } from '@angular/material/tabs';
import { SimulationControlFormComponent } from '../simulation-control-form/simulation-control-form.component';
import { SimulationPlayMode } from '../simulation.types';

@Component({
  selector: 'ml-lenia',
  standalone: true,
  imports: [
    CommonModule,
    HeatmapComponent,
    MatTabsModule,
    SimulationControlFormComponent,
  ],
  templateUrl: './lenia.component.html',
  styleUrl: './lenia.component.scss',
})
export class LeniaComponent implements OnInit {
  lenia = lenia();
  size = this.lenia.size();
  animationFrameId?: number;

  state!: number[];
  convolutedState!: number[];
  kernel!: number[];

  ngOnInit(): void {
    const statePointer = this.lenia.state();
    const convolutedStatePointer = this.lenia.convoluted_state();
    const kernelPointer = this.lenia.convolution_kernel();
    this.state = Array.from(
      new Float64Array(memory.buffer, statePointer, this.size * this.size)
    );
    this.convolutedState = Array.from(
      new Float64Array(
        memory.buffer,
        convolutedStatePointer,
        this.size * this.size
      )
    );
    this.kernel = Array.from(
      new Float64Array(memory.buffer, kernelPointer, 26 * 26)
    );
  }

  /**
   * Set the game to the updated play mode
   *
   * @param playMode - the play mode selected by the user
   */
  updatePlayMode(playMode: SimulationPlayMode) {
    switch (playMode) {
      case 'play':
        this.play();
        break;
      case 'pause':
        this.pause();
        break;
    }
  }

  /**
   * Set the game to 'play' mode
   *
   * It is done by starting a loop with requestAnimationFrame, and
   * ticking the universe at each iteration
   */
  play() {
    this.run();

    this.animationFrameId = requestAnimationFrame(this.play.bind(this));
  }

  /**
   * Set the game to 'pause' mode
   */
  pause() {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = undefined;
    }
  }

  /**
   * Play only one tick of the universe and render it
   */
  next() {
    this.run();
  }

  run() {
    this.lenia.evolve();
    const statePointer = this.lenia.state();
    const convolutedStatePointer = this.lenia.convoluted_state();
    this.state = Array.from(
      new Float64Array(memory.buffer, statePointer, this.size * this.size)
    );
    this.convolutedState = Array.from(
      new Float64Array(
        memory.buffer,
        convolutedStatePointer,
        this.size * this.size
      )
    );
  }
}
