import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';

export interface MusicFile {
  name: string;
  path: string;
}

@Injectable({
  providedIn: 'root'
})
export class MusicFileService {

  // Define the files you have placed in your /assets folder
  private readonly musicFiles: MusicFile[] = [
    // { name: 'All as One', path: '/assets/music/All_as_One_sampled_48.mp3' },
    { name: 'Away', path: '/assets/music/Away_sampled_48.mp3' },
  ];

  constructor() { }

  getMusicFiles(): Observable<MusicFile[]> {
    // Log the names of the retrieved files to the console
    console.log('--- Retrieved Music Files ---');
    this.musicFiles.forEach(file => console.log(file.name));
    console.log('-----------------------------');

    return of(this.musicFiles);
  }
}
