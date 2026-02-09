import { Component, inject, signal } from '@angular/core'
import { Router, RouterLink } from '@angular/router'
import { PassportService } from '../_services/passport-service'
import { HttpClient } from '@angular/common/http'
import { environment } from '../../environments/environment'

import { CommonModule } from '@angular/common'
import { MissionService } from '../_services/mission-service'
import { Mission } from '../_models/mission'
import { MissionSummary } from '../_models/mission-summary'

import { ThreeDTiltDirective } from '../_directives/three-d-tilt.directive'

@Component({
  selector: 'app-home',
  imports: [CommonModule, RouterLink, ThreeDTiltDirective],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home {
  _router = inject(Router)
  _passport = inject(PassportService)
  private _missionService = inject(MissionService)

  missions = signal<any[]>([])
  summary = signal<MissionSummary | undefined>(undefined)

  constructor() {
    if (!this._passport.data()) {
      this._router.navigate(['/login'])
      return
    }
    this.loadData()
  }

  async loadData() {
    try {
      const [missions, summary] = await Promise.all([
        this._missionService.getByFilter({}),
        this._missionService.getMissionSummary()
      ])

      // สุ่มรูป 1-4 ให้แต่ละภารกิจ
      const randomizedMissions = missions.map(m => ({
        ...m,
        randomImage: `/assets/card/card-img-0${Math.floor(Math.random() * 4) + 1}.jpg`
      }));

      this.missions.set(randomizedMissions as any)
      this.summary.set(summary)
    } catch (error) {
      console.error('Error loading data:', error)
    }
  }

  async joinMission(missionId: number) {
    try {
      await this._missionService.joinMission(missionId)
      this._router.navigate(['/missions', missionId])
    } catch (error) {
      console.error('Error joining mission:', error)
    }
  }

  getStackedCardImage(index: number) {
    const images = [
      '/assets/card/card-img-01.jpg',
      '/assets/card/card-img-02.jpg',
      '/assets/card/card-img-03.jpg',
      '/assets/card/card-img-04.jpg'
    ]
    return images[index % images.length]
  }

  getMissionImage(mission: any) {
    // Existing logic or fallback to stacked images if mission image is not set
    return mission.image_url || this.getStackedCardImage(0)
  }

  getRankTitle(missions: number): string {
    if (missions >= 50) return 'Challenger'
    if (missions >= 20) return 'Grandmaster'
    if (missions >= 10) return 'Master'
    if (missions >= 5) return 'Diamond'
    return 'Platinum'
  }
}
