import { ComponentFixture, TestBed } from '@angular/core/testing';

import { Enemies } from './enemies';

describe('Enemies', () => {
  let component: Enemies;
  let fixture: ComponentFixture<Enemies>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Enemies],
    }).compileComponents();

    fixture = TestBed.createComponent(Enemies);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
