import { HttpInterceptorFn } from "@angular/common/http";
import { ErrorService } from "../_services/error-service";
import { inject } from "@angular/core";
import { catchError } from "rxjs";

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
  const errorService = inject(ErrorService);
  return next(req).pipe(catchError(error => errorService.handleError(error)));
}