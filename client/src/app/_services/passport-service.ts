import { HttpClient } from '@angular/common/http'
import { inject, Injectable, signal, computed } from '@angular/core'
import { environment } from '../../environments/environment' ///
import { LoginModel, Passport, RegisterModel } from '../_models/passport'
import { firstValueFrom } from 'rxjs'
import { H } from '@angular/cdk/keycodes'
import { getAvatarUrl } from '../_helpers/util'

@Injectable({
  providedIn: 'root',
})
export class PassportService {
  private _key = 'passport'
  private _base_url = environment.baseUrl + '/api'
  private _http = inject(HttpClient)

  data = signal<Passport | undefined>(undefined)
  avatar = signal<string>("")

  // *เพิ่ม
  userId = computed(() => {
    const passport = this.data()
    if (!passport?.token) return undefined
    try {
      const payload = JSON.parse(atob(passport.token.split('.')[1]))
      // Check if sub is a number or string that needs parsing
      return Number(payload.sub)
    } catch {
      return undefined
    }
  })

  // Computed Stats
  xp = computed(() => {
    const p = this.data();
    if (!p) return 0;
    return (p.mission_success_count || 0) * 500 + (p.mission_join_count || 0) * 100;
  });

  level = computed(() => Math.floor(this.xp() / 1000) + 1);

  saveAvatarImgUrl(url: string) {
    let passport = this.data()
    if (passport) {
      passport.avatar_url = url
      this.avatar.set(url)
      this.data.set(passport)
      this.savePassportToLocalStorage()
    }
  }

  private loadPassportFormLocalStorage(): string | null {
    const jsonString = localStorage.getItem(this._key)
    if (!jsonString) return 'not found'
    try {
      const passport = JSON.parse(jsonString) as Passport
      this.data.set(passport)

      // Ensure avatar signal is synced with loaded data
      const avatar = getAvatarUrl(passport)
      this.avatar.set(avatar)
    } catch (error) {
      return `${error}`
    }
    return null
  }

  private savePassportToLocalStorage() {
    const passport = this.data()
    if (!passport) return
    const jsonString = JSON.stringify(passport)
    localStorage.setItem(this._key, jsonString)
  }

  constructor() {
    this.loadPassportFormLocalStorage()
  }

  destroy() {
    this.data.set(undefined)
    this.avatar.set("")
    localStorage.removeItem(this._key)
  }

  async get(login: LoginModel): Promise<null | string> {
    const api_url = this._base_url + '/authentication/login'
    return await this.fetchPassport(api_url, login)
  }

  async register(register: RegisterModel): Promise<null | string> {
    const api_url = this._base_url + '/brawler/register'
    return await this.fetchPassport(api_url, register)
  }

  // // *เพิ่ม
  // async recoverPassword(username: string): Promise<null | string> {
  //   const api_url = this._base_url + '/authentication/recover-password'
  //   try {
  //     await firstValueFrom(this._http.post(api_url, { username }))
  //     return null
  //   } catch (error: any) {
  //     return error.error
  //   }
  // }

  private async fetchPassport(api_url: string, model: LoginModel | RegisterModel): Promise<string | null> {
    try {
      // Use "any" to handle potential backend response structure mismatches safely
      const result = this._http.post<Passport>(api_url, model)
      const passport = await firstValueFrom(result)

      this.data.set(passport)

      // Update avatar signal immediately
      const avatar = getAvatarUrl(passport)
      this.avatar.set(avatar)

      this.savePassportToLocalStorage()
      return null
    } catch (error: any) {
      // console.error(error)
      // console.log(error.error)
      return error.error
    }

  }

}
