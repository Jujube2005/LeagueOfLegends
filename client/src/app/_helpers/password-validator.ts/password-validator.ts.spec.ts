import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PasswordValidatorTs } from './password-validator.ts';

describe('PasswordValidatorTs', () => {
  let component: PasswordValidatorTs;
  let fixture: ComponentFixture<PasswordValidatorTs>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PasswordValidatorTs]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PasswordValidatorTs);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
