// *เพิ่ม

import { Component, OnInit, inject } from '@angular/core'
import { CommonModule, AsyncPipe } from '@angular/common'
import { UserService } from '../_services/user-service'
import { Brawler } from '../_models/brawler'
import { BehaviorSubject } from 'rxjs'

import { ThreeDTiltDirective } from '../_directives/three-d-tilt.directive'

@Component({
  selector: 'app-leaderboard',
  standalone: true,
  imports: [CommonModule, AsyncPipe, ThreeDTiltDirective],
  templateUrl: './leaderboard.html',
  styleUrl: './leaderboard.scss'
})
export class LeaderboardComponent implements OnInit {
  private _userService = inject(UserService)
  leaderboard$ = new BehaviorSubject<Brawler[]>([])

  async ngOnInit() {
    try {
      const data = await this._userService.getLeaderboard()
      this.leaderboard$.next(data)
    } catch (e) {
      console.error('Failed to load leaderboard', e)
    }
  }

  getRankTitle(missions: number): string {
    if (missions >= 50) return 'Challenger'
    if (missions >= 20) return 'Grandmaster'
    if (missions >= 10) return 'Master'
    if (missions >= 5) return 'Diamond'
    return 'Platinum'
  }

  getSuccessRate(success: number, join: number): string {
    if (!join) return '0%'
    return Math.round((success / join) * 100) + '%'
  }
}
