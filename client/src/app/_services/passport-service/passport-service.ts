import { HttpClient } from "@angular/common/http";
import { inject, Injectable, signal } from "@angular/core";
import { environment } from "../../../environments/environment.development";
import { LoginModel, Passport, RegisterModel } from "../../_models/brawler/passport";
import { firstValueFrom } from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class PassportService {
  private _key = 'passport'
  private _base_url = environment.baseUrl + '/api'
  private _http = inject(HttpClient)

  data = signal<undefined | Passport>(undefined)

  private loadPassportFormLocalStorage(): string | null {
    const jsonString = localStorage.getItem(this._key)
    if (!jsonString) return 'not found'
    try {
      const passport = JSON.parse(jsonString) as Passport
      this.data.set(passport)
      console.log(passport);
      
    } catch (error) {
      return `${error}`
    }
    return null
  }
  
  private savePassportToLocalStorage(): void {
    const passport = this.data()
    if (!passport) return
    const jsonString = JSON.stringify(passport)
    localStorage.setItem(this._key, jsonString)
  }

  constructor() {
    this.loadPassportFormLocalStorage()
  }

  destroy() {
    localStorage.removeItem(this._key)
    this.data.set(undefined)
  }

  async get(login: LoginModel):Promise<null | string> {
    const api_url = this._base_url + '/authentication/login'
      return await this.fatchPassport(api_url, login)
  }

  private async fatchPassport(api_url: string, _models: LoginModel | RegisterModel): Promise<null | string> {
    try {
      const result = this._http.post<Passport>(this._base_url + '/authentication/login', _models)
      const passport = await firstValueFrom(result)
      this.data.set(passport)
      this.savePassportToLocalStorage()
      return null
    } catch (error) {
      // console.log(error)
      // console.log(Error.ERROR)
      return `${error}`
    }
  }

  async reginster(register: RegisterModel):Promise<null | string> {
    const api_url = this._base_url + '/brawlers/register'
      return await this.fatchPassport(api_url, register)

}
}