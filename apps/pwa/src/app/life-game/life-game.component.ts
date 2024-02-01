import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Cell, Universe } from '@ml/life-game';
import { memory } from '@ml/life-game/life_game_bg.wasm';
import { Subject } from 'rxjs';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

@Component({
  selector: 'ml-life-game',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './life-game.component.html',
  styleUrl: './life-game.component.css',
})
export class LifeGameComponent implements AfterViewInit {
  @ViewChild('canvas')
  canvas!: ElementRef<HTMLCanvasElement>;
  canvasContext!: CanvasRenderingContext2D;

  width = 150;
  height = 150;
  universe = Universe.new(this.width, this.height);

  errorSubject = new Subject<string>();

  ngAfterViewInit(): void {
    const canvasContext = this.initCanvasContext(this.canvas.nativeElement);
    if (canvasContext !== null) {
      this.canvasContext = canvasContext;
    } else {
      this.errorSubject.next('Can not get canvas context');
    }

    const renderLoop = () => {
      this.universe.tick();
      this.renderUniverse(this.universe, this.canvasContext);

      requestAnimationFrame(renderLoop);
    };

    // Main render loop
    this.universe.init();
    this.universe.tick();
    requestAnimationFrame(renderLoop);
  }

  initCanvasContext(canvas: HTMLCanvasElement) {
    canvas.height = (CELL_SIZE + 1) * this.height + 1;
    canvas.width = (CELL_SIZE + 1) * this.width + 1;
    return canvas.getContext('2d');
  }

  renderUniverse(universe: Universe, canvasContext: CanvasRenderingContext2D) {
    this.renderGrid(universe, canvasContext);
    this.renderCells(universe, canvasContext);
  }

  renderGrid(universe: Universe, canvasContext: CanvasRenderingContext2D) {
    const width = universe.width();
    const height = universe.height();

    canvasContext.beginPath();
    canvasContext.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      canvasContext.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      canvasContext.lineTo(
        i * (CELL_SIZE + 1) + 1,
        (CELL_SIZE + 1) * height + 1
      );
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      canvasContext.moveTo(0, j * (CELL_SIZE + 1) + 1);
      canvasContext.lineTo(
        (CELL_SIZE + 1) * width + 1,
        j * (CELL_SIZE + 1) + 1
      );
    }

    canvasContext.stroke();
  }

  renderCells(universe: Universe, canvasContext: CanvasRenderingContext2D) {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(
      memory.buffer,
      cellsPtr,
      this.width * this.height
    );

    canvasContext.beginPath();

    for (let row = 0; row < this.height; row++) {
      for (let col = 0; col < this.width; col++) {
        const idx = this.getIndex(row, col);

        canvasContext.fillStyle =
          cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

        canvasContext.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    canvasContext.stroke();
  }

  getIndex(row: number, column: number) {
    return row * this.width + column;
  }
}
