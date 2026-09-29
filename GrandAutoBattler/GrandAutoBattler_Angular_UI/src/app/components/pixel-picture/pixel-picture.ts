import { Component, input } from '@angular/core';

@Component({
  selector: 'app-pixel-picture',
  standalone: true,
  imports: [],
  templateUrl: './pixel-picture.html',
  styleUrl: './pixel-picture.scss',
})
export class PixelPicture {
  src = input.required<string>();
  alt = input<string>('');
  width = input<string>('30rem');
  height = input<string>('15rem');
  pixelSize = input<string>('2px');
  dimmed = input<boolean>(true);
}
