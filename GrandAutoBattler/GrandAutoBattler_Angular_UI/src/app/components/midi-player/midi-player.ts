import { Component, OnInit, inject, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MidiPlayerService } from '../../core/services/midi-player.service';
import { SoundFontService } from '../../core/services/soundfont.service';

const MIDI_FILES = [
  'all_as_one.mid',
  'away.mid',
  'despair.mid',
  'grand.mid',
  'jakusha.mid',
  'kaijuu.mid',
  'sensou_to_sensou.mid',
];

@Component({
  selector: 'app-midi-player',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './midi-player.html',
  styleUrl: './midi-player.scss',
})
export class MidiPlayer implements OnInit {
  midiPlayer = inject(MidiPlayerService);
  soundfontService = inject(SoundFontService);

  isPlaying = this.midiPlayer.isPlaying;
  isLoading = signal(false);
  currentSongName = '';
  private playedIndices = new Set<number>();

  ngOnInit(): void {
    this.playRandomSong();
  }

  async playRandomSong(): Promise<void> {
    this.isLoading.set(true);

    if (this.playedIndices.size >= MIDI_FILES.length) {
      this.playedIndices.clear();
    }

    let index: number;
    do {
      index = Math.floor(Math.random() * MIDI_FILES.length);
    } while (this.playedIndices.has(index));

    this.playedIndices.add(index);

    const fileName = MIDI_FILES[index];
    this.currentSongName = fileName.replace('.mid', '').replace(/_/g, ' ');

    await this.midiPlayer.loadMidi(`/assets/midi/${fileName}`);
    this.isLoading.set(false);

    this.midiPlayer.play();
  }

  async playNextRandom(): Promise<void> {
    this.midiPlayer.stop();
    await this.playRandomSong();
  }

  onVolumeChange(event: Event): void {
    const target = event.target as HTMLInputElement;
    this.soundfontService.setVolume(parseFloat(target.value));
  }
}
