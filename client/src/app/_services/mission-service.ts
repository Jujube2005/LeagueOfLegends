import { inject, Injectable } from '@angular/core'
import { environment } from '../../environments/environment'
import { HttpClient } from '@angular/common/http'
import { MissionFilter } from '../_models/mission-filter'
import { firstValueFrom, timeout } from 'rxjs'
import { Mission } from '../_models/mission'
import { AddMission } from '../_models/add-mission'
import { MissionSummary } from '../_models/mission-summary'
import { MissionMessage } from '../_models/mission-message'


@Injectable({
  providedIn: 'root',
})
export class MissionService {
  ///view
  private _base_url = environment.baseUrl + '/api'
  private _http = inject(HttpClient)

  filter: MissionFilter = {}

  async getByFilter(filter: MissionFilter): Promise<Mission[]> {
    const queryString = this.createQueryString(filter)
    const url = this._base_url + '/view/filter?' + queryString
    const missions = await firstValueFrom(this._http.get<Mission[]>(url))
    return missions
  }

  private createQueryString(filter: MissionFilter): string {
    this.filter = filter
    const params: string[] = []

    if (filter.name && filter.name.trim()) {
      params.push(`name=${encodeURIComponent(filter.name.trim())}`)
    }
    if (filter.status) {
      params.push(`status=${encodeURIComponent(filter.status)}`)
    }
    if (filter.category && filter.category.trim()) {
      params.push(`category=${encodeURIComponent(filter.category.trim())}`)
    }

    return params.join("&")
  }

  async add(mission: AddMission): Promise<number> {
    const url = this._base_url + '/mission-management'
    const observable = this._http.post<{ mission_id: number }>(url, mission)
    const resp = await firstValueFrom(observable)
    return resp.mission_id
  }

  async edit(id: number, mission: AddMission): Promise<void> {
    const url = this._base_url + '/mission-management/' + id
    await firstValueFrom(this._http.patch(url, mission))
  }

  async delete(id: number): Promise<void> {
    const url = this._base_url + '/mission-management/' + id
    await firstValueFrom(this._http.delete(url))
  }

  async getMyMissions(): Promise<Mission[]> {
    const url = this._base_url + '/brawler/my-missions'
    console.log('get ' + url)
    const observable = this._http.get<Mission[]>(url)
    const missions = await firstValueFrom(observable)
    return missions
  }

  // *เพิ่ม 
  async joinMission(mission_id: number): Promise<void> {
    const url = `${this._base_url}/crew/join/${mission_id}`
    await firstValueFrom(this._http.post<void>(url, {}))
  }

  // *เพิ่ม 
  async leaveMission(mission_id: number): Promise<void> {
    const url = `${this._base_url}/crew/leave/${mission_id}`
    await firstValueFrom(this._http.delete<void>(url))
  }

  // *เพิ่ม 
  async getJoinedMissions(): Promise<Mission[]> {
    const url = this._base_url + '/view/joined'
    const missions = await firstValueFrom(this._http.get<Mission[]>(url))
    return missions
  }

  // *เพิ่ม 
  async getMissionSummary(): Promise<MissionSummary> {
    const url = this._base_url + '/brawler/mission-summary'
    const summary = await firstValueFrom(this._http.get<MissionSummary>(url))
    return summary
  }

  // *เพิ่ม
  async startMission(mission_id: number): Promise<void> {
    const url = `${this._base_url}/mission/in-progress/${mission_id}`
    await firstValueFrom(this._http.patch<void>(url, {}))
  }

  // *เพิ่ม
  async completeMission(mission_id: number): Promise<void> {
    const url = `${this._base_url}/mission/to-completed/${mission_id}`
    await firstValueFrom(this._http.patch<void>(url, {}))
  }

  async getMission(id: number): Promise<Mission> {
    return firstValueFrom(this._http.get<Mission>(`${this._base_url}/view/${id}`).pipe(timeout(5000)))
  }

  async getCrew(missionId: number): Promise<any[]> {
    return firstValueFrom(this._http.get<any[]>(`${this._base_url}/view/crew/${missionId}`).pipe(timeout(5000)))
  }

  async kickCrew(missionId: number, memberId: number): Promise<void> {
    await firstValueFrom(this._http.post(`${this._base_url}/crew/kick/${missionId}`, { member_id: memberId }))
  }

  async transferOwnership(missionId: number, newChiefId: number): Promise<void> {
    await firstValueFrom(this._http.patch(`${this._base_url}/mission-management/${missionId}/transfer`, { new_chief_id: newChiefId }))
  }

  // *Chat
  async getMessages(missionId: number): Promise<MissionMessage[]> {
    const url = `${this._base_url}/mission-chat/${missionId}/messages`
    return firstValueFrom(this._http.get<MissionMessage[]>(url))
  }

  async sendMessage(missionId: number, content: string): Promise<void> {
    const url = `${this._base_url}/mission-chat/${missionId}/messages`
    await firstValueFrom(this._http.post<void>(url, { content }))
  }
}
