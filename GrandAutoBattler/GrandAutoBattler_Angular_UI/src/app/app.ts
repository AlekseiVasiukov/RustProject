import { Component, signal, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { NavigationComponent } from './components/navigation/navigation';

import { MusicPlayerComponent } from './components/music-player/music-player';

@Component({
  selector: 'app-root',
  // standalone: true,
  imports: [RouterOutlet, NavigationComponent, MusicPlayerComponent],
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App {
  protected readonly title = signal('GrandAutoBattler_Angular_UI');
}
