import { Component, computed, inject, signal, Signal } from '@angular/core'
import { RouterLink } from '@angular/router'
import { MissionService } from '../_services/mission-service'
import { MissionFilter } from '../_models/mission-filter'
import { Mission } from '../_models/mission'
import { FormsModule } from '@angular/forms'
import { BehaviorSubject } from 'rxjs'
import { AsyncPipe, DatePipe } from '@angular/common'
import { PassportService } from '../_services/passport-service'

@Component({
  selector: 'app-missions',
  imports: [FormsModule, AsyncPipe, DatePipe, RouterLink],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions {
  private _mission = inject(MissionService)
  private _passport = inject(PassportService)
  filter: MissionFilter = {}
  // missions: Mission[] = []

  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly missions$ = this._missionsSubject.asObservable()
  isSignin: Signal<boolean>
  userId = this._passport.userId
  isLoading = signal(true)

  constructor() {
    this.isSignin = computed(() => this._passport.data() !== undefined)
    this.filter = this._mission.filter
    this.loadMyMission()
  }

  private async loadMyMission() {
    this.isLoading.set(true)
    try {
      const missions = await this._mission.getByFilter(this.filter)
      this._missionsSubject.next(missions)
    } finally {
      this.isLoading.set(false)
    }
  }
  async onSubmit() {
    this.loadMyMission()
  }

  // *เพิ่ม
  async joinMission(mission_id: number) {
    try {
      await this._mission.joinMission(mission_id)
      await this.loadMyMission()
    } catch (e: any) {
      alert(e?.error?.message ?? 'Join failed')
    }
  }

  // *เพิ่ม
  // async leaveMission(mission_id: number) {
  //   if (!confirm('Are you sure you want to leave this mission?')) return
  //   try {
  //     await this._mission.leaveMission(mission_id)
  //     await this.loadMyMission()
  //   } catch (e: any) {
  //     alert(e?.error?.message ?? 'Leave failed')
  //   }
  // }
}