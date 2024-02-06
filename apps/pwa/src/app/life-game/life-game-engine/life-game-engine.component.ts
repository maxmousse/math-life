import {
  AfterViewInit,
  Component,
  ElementRef,
  Input,
  ViewChild,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { Cell, Universe } from '@ml/life_game';
import { memory } from '@ml/life_game/life_game_bg.wasm';
import { Subject } from 'rxjs';
import { LifeGameControlFormComponent } from '../life-game-control-form/life-game-control-form.component';
import { LifeGamePlayMode } from '../life-game.types';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

@Component({
  selector: 'ml-life-game-engine',
  standalone: true,
  templateUrl: './life-game-engine.component.html',
  styleUrl: './life-game-engine.component.css',
  imports: [CommonModule, LifeGameControlFormComponent],
})
export class LifeGameEngineComponent implements AfterViewInit {
  @Input()
  width = 100;
  @Input()
  height = 100;

  @ViewChild('canvas')
  canvas!: ElementRef<HTMLCanvasElement>;
  canvasContext!: CanvasRenderingContext2D;
  animationFrameId?: number;

  universe = Universe.new(this.width, this.height);

  errorSubject = new Subject<string>();

  ngAfterViewInit(): void {
    const canvasContext = this.initCanvasContext(this.canvas.nativeElement);
    if (canvasContext !== null) {
      this.canvasContext = canvasContext;
    } else {
      this.errorSubject.next('Can not get canvas context');
    }

    this.universe.init();
    this.renderUniverse(this.universe, this.canvasContext);
  }

  play() {
    this.universe.tick();
    this.renderUniverse(this.universe, this.canvasContext);

    this.animationFrameId = requestAnimationFrame(this.play.bind(this));
  }

  pause() {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = undefined;
    }
  }

  next() {
    this.universe.tick();
    this.renderUniverse(this.universe, this.canvasContext);

    // eslint-disable-next-line @typescript-eslint/no-empty-function
    this.animationFrameId = requestAnimationFrame(() => {});
  }

  updatePlayMode(playMode: LifeGamePlayMode) {
    switch (playMode) {
      case 'play':
        this.play();
        break;
      case 'pause':
        this.pause();
        break;
    }
  }

  toggleCell(event: MouseEvent) {
    // Ignore canvas interaction while in play mode
    if (this.isPlay()) return;

    // Else, toggle the clicked cells
    const canvas = this.canvas.nativeElement;
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(
      Math.floor(canvasTop / (CELL_SIZE + 1)),
      this.height - 1
    );
    const col = Math.min(
      Math.floor(canvasLeft / (CELL_SIZE + 1)),
      this.width - 1
    );

    this.universe.toggle_cell(row, col);

    this.renderUniverse(this.universe, this.canvasContext);
  }

  isPause() {
    return this.animationFrameId === undefined;
  }

  isPlay() {
    return this.animationFrameId !== undefined;
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
