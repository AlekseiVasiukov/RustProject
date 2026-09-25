import { Component, inject } from '@angular/core';
import { Route, Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-new-game',
  standalone: true,
  imports: [],
  templateUrl: './new-game.html',
  styleUrl: './new-game.scss',
})
export class NewGame {
  session = inject(GameSessionService);
  router = inject(Router);

  onStartNewGame() {
    this.session.startNewGame();

    //by this command, we are starting the timer.
    //if data arive - we switch to /unit-list
    setTimeout(() => {
      this.router.navigate(['/unit-list']);
    }, 100);
  }
}
