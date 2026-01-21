import { TestBed } from '@angular/core/testing';

import { PassportServicdTs } from './passport-servicd.ts';

describe('PassportServicdTs', () => {
  let service: PassportServicdTs;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PassportServicdTs);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
