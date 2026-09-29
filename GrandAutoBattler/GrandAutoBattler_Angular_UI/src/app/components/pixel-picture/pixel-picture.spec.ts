import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PixelPicture } from './pixel-picture';

describe('PixelPicture', () => {
  let component: PixelPicture;
  let fixture: ComponentFixture<PixelPicture>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PixelPicture],
    }).compileComponents();

    fixture = TestBed.createComponent(PixelPicture);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
