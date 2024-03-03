import {
  AfterViewInit,
  Component,
  ElementRef,
  Input,
  OnChanges,
  SimpleChanges,
  ViewChild,
} from '@angular/core';
import { CommonModule } from '@angular/common';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';

@Component({
  selector: 'ml-heatmap',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './heatmap.component.html',
  styleUrl: './heatmap.component.scss',
})
export class HeatmapComponent implements AfterViewInit, OnChanges {
  @Input()
  width!: number;
  @Input()
  height!: number;
  @Input()
  data!: number[];

  @ViewChild('canvas')
  canvas!: ElementRef<HTMLCanvasElement>;
  canvasContext!: CanvasRenderingContext2D;

  ngAfterViewInit(): void {
    // Get canvas and canvas context
    const canvasContext = this.initCanvasContext(this.canvas.nativeElement);

    if (canvasContext !== null) {
      this.canvasContext = canvasContext;
    } else {
      throw new Error('Can not get canvas context');
    }

    // Init universe and make a first render
    this.render();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (this.canvasContext) {
      if (changes['data']) {
        this.render();
      }
    }
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

  render() {
    //  this.renderGrid();
    this.renderCells();
  }

  renderGrid() {
    this.canvasContext.beginPath();
    this.canvasContext.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= this.width; i++) {
      this.canvasContext.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      this.canvasContext.lineTo(
        i * (CELL_SIZE + 1) + 1,
        (CELL_SIZE + 1) * this.height + 1
      );
    }

    // Horizontal lines.
    for (let j = 0; j <= this.height; j++) {
      this.canvasContext.moveTo(0.0, j * (CELL_SIZE + 1) + 1);
      this.canvasContext.lineTo(
        (CELL_SIZE + 1) * this.width + 1,
        j * (CELL_SIZE + 1) + 1
      );
    }

    this.canvasContext.stroke();
  }

  renderCells() {
    this.canvasContext.beginPath();

    for (let row = 0; row < this.height; row++) {
      for (let col = 0; col < this.width; col++) {
        const idx = this.getIndex(row, col);

        this.canvasContext.fillStyle = this.valueToColor(this.data[idx]);

        this.canvasContext.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    this.canvasContext.stroke();
  }

  getIndex(row: number, column: number) {
    return row * this.width + column;
  }

  valueToColor(value: number) {
    // Set the hue to change from red to blue
    const hue = (1 - value) * 240;

    // Set saturation and lightness to fixed values for a vibrant color
    const saturation = 100;
    const lightness = 50;

    // Convert HSL to RGB
    const rgb = this.hslToRgb(hue, saturation, lightness);

    return `rgb(${rgb[0]}, ${rgb[1]}, ${rgb[2]})`;
  }

  // Helper function to convert HSL to RGB
  hslToRgb(h: number, s: number, l: number) {
    h /= 360;
    s /= 100;
    l /= 100;

    let r: number;
    let g: number;
    let b: number;

    if (s === 0) {
      r = g = b = l; // achromatic
    } else {
      const hue2rgb = (p: number, q: number, t: number) => {
        if (t < 0) t += 1;
        if (t > 1) t -= 1;
        if (t < 1 / 6) return p + (q - p) * 6 * t;
        if (t < 1 / 2) return q;
        if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
        return p;
      };

      const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
      const p = 2 * l - q;

      r = hue2rgb(p, q, h + 1 / 3);
      g = hue2rgb(p, q, h);
      b = hue2rgb(p, q, h - 1 / 3);
    }

    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
  }
}
