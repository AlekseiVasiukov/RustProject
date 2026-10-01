import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CombatantDetails } from './combatant-details';

describe('CombatantDetails', () => {
  let component: CombatantDetails;
  let fixture: ComponentFixture<CombatantDetails>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CombatantDetails],
    }).compileComponents();

    fixture = TestBed.createComponent(CombatantDetails);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
