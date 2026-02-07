import { Component, computed, inject, Signal, signal, effect } from '@angular/core'
import { MatButtonModule } from '@angular/material/button'
import { MatToolbarModule } from '@angular/material/toolbar'
import { PassportService } from '../_services/passport-service'
import { MatMenuModule } from '@angular/material/menu'
import { getAvatarUrl } from '../_helpers/util'
import { Router, RouterLink, RouterLinkActive } from "@angular/router"
import { MatIconModule } from '@angular/material/icon'
import { MatBadgeModule } from '@angular/material/badge'
import { InviteService } from '../_services/invite.service'
import { NotificationService } from '../_services/notification-service'

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, MatMenuModule, RouterLink, RouterLinkActive, MatIconModule, MatBadgeModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',
})
export class Navbar {
  private _router = inject(Router)
  private _passport = inject(PassportService)
  private _inviteService = inject(InviteService)
  private _notificationService = inject(NotificationService)

  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  invite_count = computed(() => this._inviteService.invites().length)

  constructor() {
    this.display_name = computed(() => this._passport.data()?.display_name)
    this.avatar_url = computed(() => this._passport.avatar())

    effect(() => {
      if (this.display_name()) {
        this.updateInviteCount()
      }
    })

    this._notificationService.notifications$.subscribe(() => {
      this.updateInviteCount()
    })
  }

  async updateInviteCount() {
    try {
      await this._inviteService.getMyInvites();
    } catch (e) {
      console.error('Failed to fetch invites', e);
    }
  }

  logout() {
    this._passport.destroy()

    this._router.navigate(['/login'])
  }
}
