import { Injectable, signal, inject, DestroyRef } from '@angular/core';
import { Subject, Observable } from 'rxjs';
// import * as MidiPlayer from 'midi-player-js';
// import Player from 'midi-player-js';
// import Event from 'midi-player-js';
import * as MidiPlayer from 'midi-player-js';
import { EventType } from '@angular/router';

// Extract the actual class from the module namespace
const PlayerClass = (MidiPlayer as any).default || MidiPlayer;
// const EventClass = (Event as any).default || MidiPlayer;
type MidiEvent = MidiPlayer.Event;
type MidiTrack = MidiPlayer.Track;

export interface MidiNoteEvent {
  note: number; // MIDI note number (0-127)
  velocity: number; // 0-127
  noteName: string; //  "C4",
  isNoteOn: boolean;
}

@Injectable({
  providedIn: 'root',
})
export class MidiPlayerService {
  private destroyRef = inject(DestroyRef);

  // Reactive state
  private _isPlaying = signal<boolean>(false);
  isPlaying = this._isPlaying.asReadonly();

  // Event stream for note-on / note-off
  private noteSubject = new Subject<MidiNoteEvent>();
  noteEvents$: Observable<MidiNoteEvent> = this.noteSubject.asObservable();

  // Internal player instance
  private player: any;
  // private player: InstanceType<typeof PlayerClass> | null = null;
  constructor() {
    this.initPlayer();
  }

  private initPlayer(): void {
    this.player = new PlayerClass((event: MidiEvent) => {
      if (event.name === 'Note on' || event.name === 'Note off') {
        this.noteSubject.next({
          note: event.noteNumber ?? 0,
          velocity: event.velocity ?? 0,
          noteName: event.noteName ?? event.note ?? 'C4',
          isNoteOn: event.name === 'Note on' && (event.velocity ?? 0) > 0,
        });
      }
    });

    // Only use valid events from PlayerEventType
    this.player.on('playing', () => {
      this._isPlaying.set(true);
    });

    this.player.on('endOfFile', () => {
      this._isPlaying.set(false);
    });
  }

  //  Load a MIDI file from an asset path.
  loadMidi(assetPath: string): Promise<void> {
    return fetch(assetPath)
      .then((response) => {
        if (!response.ok) throw new Error(`Failed to load MIDI: ${assetPath}`);
        return response.arrayBuffer();
      })
      .then((buffer) => {
        this.player?.loadArrayBuffer(buffer);
      });
  }

  play(): void {
    this.player?.play();
  }

  stop(): void {
    this.player?.stop();
    this._isPlaying.set(false);
  }

  destroy(): void {
    this.stop();
    this.noteSubject.complete();
  }
}
