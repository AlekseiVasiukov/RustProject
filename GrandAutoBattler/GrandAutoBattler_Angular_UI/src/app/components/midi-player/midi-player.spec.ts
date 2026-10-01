import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MidiPlayer } from './midi-player';

describe('MidiPlayer', () => {
  let component: MidiPlayer;
  let fixture: ComponentFixture<MidiPlayer>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MidiPlayer],
    }).compileComponents();

    fixture = TestBed.createComponent(MidiPlayer);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
