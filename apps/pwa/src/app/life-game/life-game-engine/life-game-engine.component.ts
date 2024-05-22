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
import { SimulationControlFormComponent } from '../../simulation-control-form/simulation-control-form.component';
import { SimulationPlayMode } from '../../simulation.types';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

@Component({
  selector: 'ml-life-game-engine',
  standalone: true,
  templateUrl: './life-game-engine.component.html',
  styleUrl: './life-game-engine.component.scss',
  imports: [CommonModule, SimulationControlFormComponent],
})
export class LifeGameEngineComponent implements AfterViewInit {
  @Input()
  width!: number;
  @Input()
  height!: number;
  @Input()
  pattern!: string;

  @ViewChild('canvas')
  canvas!: ElementRef<HTMLCanvasElement>;
  canvasContext!: CanvasRenderingContext2D;
  animationFrameId?: number;
  lastFrameTimestamp?: number;
  fps?: number;

  universe!: Universe;

  ngAfterViewInit(): void {
    // Get canvas and canvas context
    const canvasContext = this.initCanvasContext(this.canvas.nativeElement);

    if (canvasContext !== null) {
      this.canvasContext = canvasContext;
    } else {
      throw new Error('Can not get canvas context');
    }

    // Init universe and make a first render
    this.universe = Universe.new(this.width, this.height);
    this.universe.init(this.pattern);
    this.renderUniverse(this.universe, this.canvasContext);
  }

  /**
   * Set the game to 'play' mode
   *
   * It is done by starting a loop with requestAnimationFrame, and
   * ticking the universe at each iteration
   */
  play() {
    this.universe.tick();
    this.renderUniverse(this.universe, this.canvasContext);

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
    this.universe.tick();
    this.renderUniverse(this.universe, this.canvasContext);

    // eslint-disable-next-line @typescript-eslint/no-empty-function
    this.animationFrameId = requestAnimationFrame(() => {});
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
   * Toggle the state of the cell clicked by the user
   * (only if the game is not in play mode)
   *
   * @param event - Mouse event that contains the click coordinates
   */
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

  /**
   * Denote if the game is in 'pause' mode
   */
  isPause() {
    return this.animationFrameId === undefined;
  }

  /**
   * Denote if the game is in 'play' mode
   */
  isPlay() {
    return this.animationFrameId !== undefined;
  }

  /**
   * Set the size of the canvas to the game size and
   * get a 2d context
   *
   * @param canvas - the canvas to initiate
   * @returns a ref to the canvas context
   */
  initCanvasContext(canvas: HTMLCanvasElement) {
    canvas.height = (CELL_SIZE + 1) * this.height + 1;
    canvas.width = (CELL_SIZE + 1) * this.width + 1;
    return canvas.getContext('2d');
  }

  renderUniverse(universe: Universe, canvasContext: CanvasRenderingContext2D) {
    // FPS calculation
    const now = performance.now();
    this.fps = this.lastFrameTimestamp
      ? (1 / (now - this.lastFrameTimestamp)) * 1000
      : undefined;
    this.lastFrameTimestamp = now;

    // Rendering
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
