import { Component, computed, inject, Signal, OnInit, ChangeDetectorRef } from '@angular/core'
import { getAvatarUrl } from '../_helpers/util'
import { PassportService } from '../_services/passport-service'
import { MatDialog } from '@angular/material/dialog'
import { UploadImg } from '../_dialogs/upload-img/upload-img'
import { UserService } from '../_services/user-service'
import { AchievementService } from '../_services/achievement-service'
import { Achievement } from '../_models/achievement'
import { DatePipe, CommonModule } from '@angular/common'

import { Mission } from '../_models/mission'
import { InviteService } from '../_services/invite.service'
import { EditProfileDialog } from '../_dialogs/edit-profile/edit-profile'

@Component({
  selector: 'app-profile',
  imports: [DatePipe, CommonModule],
  templateUrl: './profile.html',
  styleUrl: './profile.scss',
})
export class Profile implements OnInit {
  avatar_url: Signal<string>
  display_name: Signal<string | undefined>
  achievements: Achievement[] = []
  invites: any[] = [] // List of pending invites
  missions: Mission[] = [] // List of user's missions
  systemLogs = [
    { type: 'success', message: 'Neural Link Established', time: new Date() },
    { type: 'info', message: 'System Diagnostics: Optimal', time: new Date(Date.now() - 3600000 * 2) },
    { type: 'warning', message: 'Energy Reserves at 94%', time: new Date(Date.now() - 3600000 * 5) },
    { type: 'info', message: 'Protocol v2.4.0 Loaded', time: new Date(Date.now() - 3600000 * 24) }
  ];

  _passport = inject(PassportService)
  private _dialog = inject(MatDialog)
  private _user = inject(UserService)
  private _achievement = inject(AchievementService)
  private _invite = inject(InviteService)
  private _cdr = inject(ChangeDetectorRef)

  constructor() {
    this.avatar_url = this._passport.avatar
    this.display_name = computed(() => this._passport.data()?.display_name)
  }

  // Computed Stats
  xp = this._passport.xp;
  level = this._passport.level;

  nextLevelXp = computed(() => this.level() * 1000);
  currentLevelProgress = computed(() => {
    const currentLevelXp = this.xp() % 1000;
    return (currentLevelXp / 1000) * 100;
  });

  ngOnInit() {
    this.loadAchievements()
    this.loadInvites()
    this.loadMissions()
  }

  // *เพิ่ม
  async loadAchievements() {
    try {
      this.achievements = await this._achievement.getAchievements()
      this._cdr.detectChanges()
    } catch (e) {
      console.error('Failed to load achievements', e)
    }
  }

  async loadMissions() {
    try {
      this.missions = await this._user.getMyMissions()
      this._cdr.detectChanges()
    } catch (e) {
      console.error('Failed to load missions', e)
    }
  }

  async loadInvites() {
    try {
      this.invites = await this._invite.getMyInvites()
      this._cdr.detectChanges()
    } catch (e) {
      console.error('Failed to load invites', e)
    }
  }

  async acceptInvite(inviteId: number) {
    if (!confirm('Accept this mission invite?')) return;
    try {
      await this._invite.accept(inviteId);
      await this.loadInvites(); // Reload list
      // Optionally redirect to mission or show success
      alert('Joined mission successfully!');
    } catch (e: any) {
      console.error(e);
      alert(e?.error?.message || e?.message || 'Failed to accept invite');
    } finally {
      this._cdr.detectChanges();
    }
  }

  async declineInvite(inviteId: number) {
    if (!confirm('Decline this mission invite?')) return;
    try {
      await this._invite.decline(inviteId);
      await this.loadInvites(); // Reload list
    } catch (e: any) {
      alert(e?.error || 'Failed to decline invite');
    }
  }

  openDialog() {
    const ref = this._dialog.open(UploadImg)
    ref.afterClosed().subscribe(async file => {
      if (file) {
        const error = await this._user.uploadAvatarImg(file)
        if (error)
          console.error(error)
      }
    })
  }

}

