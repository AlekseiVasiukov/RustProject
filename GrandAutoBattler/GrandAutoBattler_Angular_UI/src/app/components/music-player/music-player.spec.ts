import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MusicPlayer } from './music-player';

describe('MusicPlayer', () => {
  let component: MusicPlayer;
  let fixture: ComponentFixture<MusicPlayer>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MusicPlayer],
    }).compileComponents();

    fixture = TestBed.createComponent(MusicPlayer);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
