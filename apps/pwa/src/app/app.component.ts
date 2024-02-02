import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatGridListModule } from '@angular/material/grid-list';
@Component({
  standalone: true,
  imports: [
    RouterModule,
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MatGridListModule,
  ],
  selector: 'ml-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {}
