import { Injectable, inject, signal, DestroyRef } from '@angular/core';
import { MidiPlayerService } from './midi-player.service';
import * as SoundFont from 'soundfont-player';

@Injectable({
  providedIn: 'root',
})
export class SoundFontService {
  private midiPlayer = inject(MidiPlayerService);
  private destroyRef = inject(DestroyRef);

  private audioContext: AudioContext | null = null;
  private piano: SoundFont.Player | null = null;

  private _volume = signal<number>(0.5);
  volume = this._volume.asReadonly();

  constructor() {
    this.subscribeToMidiEvents();
    console.log('Soundfont constructed');
  }

  private ensureAudioContext(): void {
    if (!this.audioContext) {
      this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    }
    if (this.audioContext.state === 'suspended') {
      this.audioContext.resume();
    }
  }

async loadInstrument(): Promise<void> {
  this.ensureAudioContext();

  console.log('AudioContext:', this.audioContext?.state);

  if (!this.audioContext || this.piano) return;

  this.piano = await SoundFont.instrument(
    this.audioContext,
    'acoustic_grand_piano',
    {
      gain: this._volume(),
    }
  );

  console.log('Piano loaded:', this.piano);
  console.log('AudioContext after load:', this.audioContext.state);
}

 private subscribeToMidiEvents(): void {
  this.midiPlayer.noteEvents$.subscribe((event) => {

    // console.log('MIDI NOTE:', event);

    if (!this.piano || !this.audioContext) {
      console.warn('No piano or AudioContext');
      return;
    }

    if (event.isNoteOn) {
      const gain = this._volume() * (event.velocity / 127);

      console.log(
        'PLAY:',
        event.noteName,
        'gain:',
        gain,
        'context:',
        this.audioContext.state
      );

      this.piano.play(
        event.noteName,
        this.audioContext.currentTime,
        {
          gain,
        }
      );
    }
  });
}

  setVolume(value: number): void {
    this._volume.set(Math.max(0, Math.min(1, value)));
  }
}
