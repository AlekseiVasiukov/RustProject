import { Component, inject, computed, Inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-game-result',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './game-result.html',
  styleUrl: './game-result.scss',
})
export class GameResult {
  session = inject(GameSessionService);
  router = inject(Router);
  //0 -we, 1 - they
  weWon = computed(() => {
    const result = this.session.gameResult();
    return result?.battle_journal?.winner_side === 0;
  });

  resultMessage = computed(() => {
    return this.weWon() ? 'You won!' : 'You lose!';
  });

  onStartNewGame() {
    this.session.startNewGame();
    this.router.navigate(['/new-game']);
  }
}
