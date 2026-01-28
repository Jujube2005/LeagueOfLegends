import { CanActivateFn, Router } from '@angular/router';
import { inject } from '@angular/core';
import { PassportService } from '../_services/passport-service/passport-service';

export const authGuard: CanActivateFn = () => {
  const passport = inject(PassportService);
  const router = inject(Router);

  if (passport.data()?.access_token) {
    return true;
  }

  return router.parseUrl('/login');
};
