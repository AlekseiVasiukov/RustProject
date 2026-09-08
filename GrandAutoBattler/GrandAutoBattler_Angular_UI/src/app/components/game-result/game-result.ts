import { Component, inject, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MockDataService } from '../../core/services/mock-data.service';

@Component({
  selector: 'app-game-result',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './game-result.html',
  styleUrl: './game-result.scss',
})
export class GameResult {
  private mockDataService = inject(MockDataService);

  result = signal<any>(null);

  constructor() {
    // For demonstration, we fetch a result for a mock game ID
    this.mockDataService.getGameResult('mock-game-id-123').subscribe(res => {
      this.result.set(res);
    });
  }
}
