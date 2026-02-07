import { Component, inject, OnInit, ChangeDetectorRef } from '@angular/core'
import { ActivatedRoute, Router } from '@angular/router'
import { MissionService } from '../../_services/mission-service'
import { Mission } from '../../_models/mission'
import { CommonModule, Location } from '@angular/common'
import { FormsModule } from '@angular/forms'
import { PassportService } from '../../_services/passport-service'
import { MissionChatComponent } from '../../_components/mission-chat/mission-chat'
import { MatDialog } from '@angular/material/dialog'
import { InviteMemberComponent } from '../../_dialogs/invite-member/invite-member'

@Component({
  selector: 'app-mission-specific-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, MissionChatComponent],
  templateUrl: './mission-specific-manager.html',
  styleUrl: './mission-specific-manager.scss',
})
export class MissionSpecificManager implements OnInit {
  private _route = inject(ActivatedRoute)
  private _router = inject(Router)
  private _missionService = inject(MissionService)
  private _location = inject(Location)
  private _cdr = inject(ChangeDetectorRef)
  private _passport = inject(PassportService)
  private _dialog = inject(MatDialog)
  private _loadingTimer: any = null

  missionId: number = 0
  mission: Mission | undefined
  crew: any[] = []
  isLoading = false
  error: string | null = null

  // Edit form
  isEditing = false
  editName = ''
  editDescription = ''

  get isChief(): boolean {
    const userId = this._passport.userId()
    return !!(this.mission && userId && this.mission.chief_id === userId)
  }

  async ngOnInit() {
    this.missionId = Number(this._route.snapshot.paramMap.get('id'))
    if (this.missionId) {
      await this.loadData()
    }
  }

  async loadData() {
    this.isLoading = true
    this.error = null
    this._cdr.detectChanges()

    this._loadingTimer = setTimeout(() => {
      if (this.isLoading) {
        console.warn('Loading timeout triggered')
        this.error = 'The connection was faulty. Please try again.'
        this.isLoading = false
        this._cdr.detectChanges()
      }
    }, 7000)

    try {
      const [mission, crew] = await Promise.all([
        this._missionService.getMission(this.missionId),
        this._missionService.getCrew(this.missionId)
      ])

      this.mission = mission
      this.crew = crew

      this.editName = this.mission.name
      this.editDescription = this.mission.description || ''
    } catch (e: any) {
      console.error('Load data error:', e)
      this.error = e?.message || 'Failed to load mission data'
    } finally {
      if (this._loadingTimer) {
        clearTimeout(this._loadingTimer)
        this._loadingTimer = null
      }
      this.isLoading = false
      this._cdr.detectChanges()
    }
  }

  goBack() {
    this._location.back()
  }

  async startMission() {
    if (!confirm('Start mission?')) return
    try {
      await this._missionService.startMission(this.missionId)
      await this.loadData()
    } catch (e: any) {
      alert(e?.error?.message || 'Failed')
    }
  }

  async completeMission() {
    if (!confirm('Complete mission?')) return
    try {
      await this._missionService.completeMission(this.missionId)
      await this.loadData()
    } catch (e: any) {
      alert(e?.error?.message || 'Failed')
    }
  }

  async deleteMission() {
    if (!confirm('Delete this mission?')) return
    try {
      await this._missionService.delete(this.missionId)
      this._router.navigate(['/chief'])
    } catch (e: any) {
      alert(e?.error?.message || 'Failed')
    }
  }

  toggleEdit() {
    this.isEditing = !this.isEditing
    if (!this.isEditing && this.mission) {
      // Reset
      this.editName = this.mission.name
      this.editDescription = this.mission.description || ''
    }
  }

  async saveEdit() {
    if (!this.mission) return
    const payload = {
      name: this.editName.trim(),
      description: this.editDescription.trim() || undefined
    }
    try {
      await this._missionService.edit(this.missionId, payload)
      this.isEditing = false
      await this.loadData()
    } catch (e: any) {
      alert(e?.error?.message || 'Failed to save')
    }
  }

  async kick(memberId: number) {
    if (!confirm('Kick this member?')) return
    try {
      await this._missionService.kickCrew(this.missionId, memberId)
      alert('Member kicked successfully')
      await this.loadData()
    } catch (e: any) {
      alert(e?.error?.message || 'Failed to kick')
    }
  }



  async leaveMission() {
    if (!confirm('Are you sure you want to leave this mission?')) return
    try {
      await this._missionService.leaveMission(this.missionId)
      this._router.navigate(['/chief']) // Redirect to My Missions
    } catch (e: any) {
      alert(e?.error?.message || 'Leave failed')
    }
  }

  openInviteDialog() {
    this._dialog.open(InviteMemberComponent, {
      width: '400px',
      data: {
        missionId: this.missionId,
        currentMembers: this.crew.map(c => c.id).concat(this.mission?.chief_id)
      }
    })
  }
}

