import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BattleJournal } from './battle-journal';

describe('BattleJournal', () => {
  let component: BattleJournal;
  let fixture: ComponentFixture<BattleJournal>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BattleJournal],
    }).compileComponents();

    fixture = TestBed.createComponent(BattleJournal);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
