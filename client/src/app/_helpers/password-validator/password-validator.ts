import { AbstractControl, ValidatorFn,  ValidationErrors} from "@angular/forms"

export const PasswordValidator = (min: number, max: number): ValidatorFn => {
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

 export const PasswordMatchValidator = (ctrl_pw_name: string, ctrl_cf_pw_name: string): ValidatorFn => {
    return (formGroup: AbstractControl): ValidationErrors | null => {
        const ctrlPw = formGroup.get(ctrl_pw_name);
        const ctrlCfPw = formGroup.get(ctrl_cf_pw_name);
        if (!ctrlPw || !ctrlCfPw) return null;
        const isMatch = ctrlPw.value === ctrlCfPw.value;
        if (!isMatch) {
            const errors = ctrlCfPw.errors || {};
            ctrlCfPw.setErrors({ ...errors, mismatch: true });
        } else {
            if (ctrlCfPw.errors) {
                delete ctrlCfPw.errors['mismatch'];
                if (Object.keys(ctrlCfPw.errors).length === 0) {
                    ctrlCfPw.setErrors(null);
                } else {
                    ctrlCfPw.setErrors(ctrlCfPw.errors);
                }
            }
        }
        return null;
    };
};