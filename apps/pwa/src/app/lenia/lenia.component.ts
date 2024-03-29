import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { lenia } from '@ml/lenia';
import { HeatmapComponent } from '../heatmap/heatmap.component';
import { memory } from '@ml/lenia/lenia_bg.wasm';
import { MatTabsModule } from '@angular/material/tabs';

@Component({
  selector: 'ml-lenia',
  standalone: true,
  imports: [CommonModule, HeatmapComponent, MatTabsModule],
  templateUrl: './lenia.component.html',
  styleUrl: './lenia.component.scss',
})
export class LeniaComponent implements OnInit {
  lenia = lenia();
  data!: number[];

  ngOnInit(): void {
    const pointer = this.lenia.state();
    this.data = Array.from(new Float64Array(memory.buffer, pointer, 64 * 64));
  }

  run() {
    this.lenia.evolve();
    const pointer = this.lenia.state();
    this.data = Array.from(new Float64Array(memory.buffer, pointer, 64 * 64));
  }
}
