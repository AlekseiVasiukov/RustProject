import { Component, AfterViewInit, ViewChild, ElementRef } from '@angular/core';
import { MusicFileService, MusicFile } from './../../core/services/music-file-service'; // Adjust path if needed
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-music-player',
  templateUrl: './music-player.html',
  styleUrl: './music-player.scss',
})
// Use AfterViewInit instead of OnInit so the HTML template is fully loaded
export class MusicPlayerComponent implements AfterViewInit {
  @ViewChild('audioPlayer') audioPlayer!: ElementRef<HTMLAudioElement>;

  // Initial volume is 50%
  volume = 20;
  // private audioElement!: HTMLAudioElement ;

  constructor(private musicFileService: MusicFileService) {}

  ngAfterViewInit(): void {
    this.musicFileService.getMusicFiles().subscribe((files: MusicFile[]) => {
      if (files.length > 0 && this.audioPlayer) {
        console.log(files);
        console.log(files.length);
        const audioElement = this.audioPlayer.nativeElement;

        // Load the first retrieved file and set the initial volume
        audioElement.src = files[0].path;
        console.log(audioElement.src);
        audioElement.volume = this.volume / 100;

        // Optional: Uncomment the line below if you want it to auto-play.
        // Note: Browsers often block auto-play without user interaction.
        // environment
        // if (environment.production) {
        //   audioElement.play().catch((e) => console.log('Auto-play prevented by browser'));
        // } else {
        //   console.log('Environment is not "production" - music is not autoplayed');
        // }
      }
    });
  }
  onPlay(event: Event): void {
        // const input = event.target as HTMLInputElement;
    if (this.audioPlayer) {
      this.audioPlayer.nativeElement.play();
    }
  }
  // onPlay(event: Event): void {
  //   // const input = event.target as HTMLInputElement;
  //   if (this.audioPlayer) {
  //     this.audioPlayer.nativeElement.play;
  //   }
  //   if (this.audioPlayer) {

  //     // HTML audio volume ranges from 0.0 to 1.0
  //     this.audioPlayer.nativeElement.volume = 50 / 100;
  //   }
  //   // const audioElement = this.audioPlayer.nativeElement;

  //   // audioElement.play();
  // }
  // onPause(event: Event): void {
  //   if (this.audioPlayer) {
  //     // HTML audio volume ranges from 0.0 to 1.0
  //     this.audioPlayer.nativeElement.pause;
  //   }
  // }
  onVolumeChange(event: Event): void {
    const input = event.target as HTMLInputElement;
    this.volume = +input.value;

    if (this.audioPlayer) {
      // HTML audio volume ranges from 0.0 to 1.0
      this.audioPlayer.nativeElement.volume = this.volume / 100;
    }
  }
}
