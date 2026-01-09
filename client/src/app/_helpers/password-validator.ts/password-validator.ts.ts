import { AbstractControl, ValidatorFn,  ValidationErrors} from "@angular/forms"

export const passwordValidatorv= (min: number, max: number): ValidatorFn => {
  return (ctrl: AbstractControl): ValidationErrors | null => {
    const password = ctrl.value as string
    if (!password) return { required: true }
    if (password.length < min || password.length > max) return { invalidLength: true}
    if (!/[A-Z]/.test(password)) return { invalidUpperCase: true }
    if (!/[a-z]/.test(password)) return { invalidLowerCase: true }
    if (!/[0-9]/.test(password)) return { invalidNumeric: true }
    if (!/[!@#$%^&*(),.?":{}|<>]/.test(password)) return { invalidSpecialCase: true }
    return null
  }
}

export const passwordMatchValidator = (ctrl_pw_name: string, ctrl_cf_pw_name: string): ValidatorFn => {
  return () => {
    return null
  }
}