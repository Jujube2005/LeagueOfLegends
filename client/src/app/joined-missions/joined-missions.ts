// *เพิ่ม 

import { Component, inject } from '@angular/core'
import { MissionService } from '../_services/mission-service'
import { Mission } from '../_models/mission'
import { AsyncPipe, DatePipe } from '@angular/common'
import { BehaviorSubject } from 'rxjs'
import { RouterLink } from '@angular/router'

@Component({
  selector: 'app-joined-missions',
  imports: [AsyncPipe, DatePipe, RouterLink],
  templateUrl: './joined-missions.html',
  styleUrl: './joined-missions.scss',
})
export class JoinedMissions {
  private _mission = inject(MissionService)

  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly missions$ = this._missionsSubject.asObservable()

  constructor() {
    this.loadMissions()
  }

  private async loadMissions() {
    try {
      const missions = await this._mission.getJoinedMissions()
      this._missionsSubject.next(missions)
    } catch (e) {
      console.error(e)
    }
  }

  async leaveMission(mission_id: number) {
    if (!confirm('Are you sure you want to leave this mission?')) return
    try {
      await this._mission.leaveMission(mission_id)
      await this.loadMissions()
    } catch (e: any) {
      alert(e?.error?.message ?? e?.error ?? 'Leave failed')
    }
  }
}
