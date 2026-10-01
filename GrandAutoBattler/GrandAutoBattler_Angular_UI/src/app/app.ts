import { Component, signal, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { NavigationComponent } from './components/navigation/navigation';
import { MidiPlayer } from './components/midi-player/midi-player';
import { SoundFontService } from './core/services/soundfont.service';
@Component({
  selector: 'app-root',
  // standalone: true,
  imports: [RouterOutlet, NavigationComponent, MidiPlayer],
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App {
  protected readonly title = signal('GrandAutoBattler_Angular_UI');
  private soundfont = inject(SoundFontService);
  // private midiPlayer = inject(this.midiPlayer);

  async onUserInteraction(): Promise<void> {
    await this.soundfont.loadInstrument();
  }
}
