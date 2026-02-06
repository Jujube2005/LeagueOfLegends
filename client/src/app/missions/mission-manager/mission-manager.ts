import { Component, inject } from '@angular/core'
import { MissionService } from '../../_services/mission-service'
import { MatDialog } from '@angular/material/dialog'
import { Mission } from '../../_models/mission'
import { NewMission } from '../../_dialogs/new-mission/new-mission'
import { AddMission } from '../../_models/add-mission'
import { MatIconModule } from '@angular/material/icon'
import { AsyncPipe, DatePipe } from '@angular/common'
import { BehaviorSubject } from 'rxjs'
import { PassportService } from '../../_services/passport-service'
import { NotificationService } from '../../_services/notification-service'
import { Router } from '@angular/router'

@Component({
  selector: 'app-mission-manager',
  imports: [MatIconModule, DatePipe, AsyncPipe],
  templateUrl: './mission-manager.html',
  styleUrl: './mission-manager.scss',
})
export class MissionManager {
  private _mission = inject(MissionService)
  private _dialog = inject(MatDialog)
  private _passport = inject(PassportService)
  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly myMissions$ = this._missionsSubject.asObservable()
  private _notification = inject(NotificationService)
  private _router = inject(Router)
  joinAlerts = new Set<number>()

  constructor() {
    this.loadMyMission()
    this._notification.notifications$.subscribe(n => {
      if (n.type === 'JoinMission' && n.metadata?.mission_id) {
        this.joinAlerts.add(Number(n.metadata.mission_id))
      }
    })
  }

  private async loadMyMission() {
    const missions = await this._mission.getMyMissions()
    this._missionsSubject.next(missions)
  }

  navigateToMission(id: number) {
    this.joinAlerts.delete(id)
    this._router.navigate(['/chief/mission', id])
  }

  hasJoinAlert(id: number) {
    return this.joinAlerts.has(id)
  }

  clearJoinAlert(id: number) {
    this.joinAlerts.delete(id)
  }

  openDialog() {
    let chief_display_name = this._passport.data()?.display_name || "unnamed"
    const ref = this._dialog.open(NewMission)
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
      if (addMission) {
        const id = await this._mission.add(addMission)
        const now = new Date()
        const newMission: Mission = {
          id,
          name: addMission.name,
          description: addMission.description,
          category: addMission.category,
          max_crew: addMission.max_crew || 5,
          status: 'Open',
          chief_id: 0,
          chief_display_name,
          crew_count: 0,
          created_at: now,
          updated_at: now
        }
        // เพิ่มข้อมูลใหม่เข้าไปใน BehaviorSubject
        const currentMissions = this._missionsSubject.value
        this._missionsSubject.next([...currentMissions, newMission])
      }
    })
  }

  // *เพิ่ม
  openEditDialog(mission: Mission) {
    const ref = this._dialog.open(NewMission, {
      data: {
        name: mission.name,
        description: mission.description,
        category: mission.category,
        max_crew: mission.max_crew
      }
    })
    ref.afterClosed().subscribe(async (updatedData: AddMission) => {
      if (updatedData) {
        try {
          await this._mission.edit(mission.id, updatedData)
          // Update local state
          const currentMissions = this._missionsSubject.value
          const index = currentMissions.findIndex(m => m.id === mission.id)
          if (index !== -1) {
            currentMissions[index] = { ...currentMissions[index], ...updatedData }
            this._missionsSubject.next([...currentMissions])
          }
        } catch (e: any) {
          alert(e?.error?.message ?? e?.error ?? 'Edit failed')
        }
      }
    })
  }

  // *เพิ่ม
  async deleteMission(mission_id: number) {
    if (!confirm('Are you sure you want to delete this mission?')) return

    try {
      await this._mission.delete(mission_id)
      // Update local state
      const currentMissions = this._missionsSubject.value.filter(m => m.id !== mission_id)
      this._missionsSubject.next(currentMissions)
    } catch (e: any) {
      alert(e?.error?.message ?? e?.error ?? 'Delete failed')
    }
  }

  // hasJoinAlert(mission_id: number): boolean {
  //   return this.joinAlerts.has(mission_id)
  // }

  // clearJoinAlert(mission_id: number) {
  //   this.joinAlerts.delete(mission_id)
  // }

  // *เพิ่ม
  async startMission(mission_id: number) {
    if (!confirm('Start this mission?')) return
    try {
      await this._mission.startMission(mission_id)
      await this.loadMyMission()
    } catch (e: any) {
      alert(e?.error?.message ?? e?.error ?? 'Start mission failed')
    }
  }

  // *เพิ่ม
  async completeMission(mission_id: number) {
    if (!confirm('Complete this mission?')) return
    try {
      await this._mission.completeMission(mission_id)
      await this.loadMyMission()
    } catch (e: any) {
      alert(e?.error?.message ?? e?.error ?? 'Complete mission failed')
    }
  }
}
