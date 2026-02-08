import { Component, computed, inject, Signal, signal, effect } from '@angular/core'
import { PassportService } from '../_services/passport-service'
import { Router, RouterLink, RouterLinkActive } from "@angular/router"
import { InviteService } from '../_services/invite.service'
import { NotificationService } from '../_services/notification-service'
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-navbar',
  imports: [RouterLink, RouterLinkActive, MatSnackBarModule, CommonModule],
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
  isHidden = signal(false)

  constructor() {
    this._router.events.subscribe(() => {
      this.isHidden.set(this._router.url.includes('/login') || this._router.url.includes('/register'))
    })
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

  /* Existing Logic expanded */
  private _snackBar = inject(MatSnackBar)

  showNotifications = signal(false)
  processing = signal<Set<number>>(new Set())

  invites = this._inviteService.invites; // Expose invites directly

  toggleNotifications() {
    this.showNotifications.update(v => !v);
  }

  isProcessing(id: number): boolean {
    return this.processing().has(id);
  }

  private setProcessing(id: number, state: boolean) {
    this.processing.update((current: Set<number>) => {
      const newSet = new Set(current);
      if (state) newSet.add(id);
      else newSet.delete(id);
      return newSet;
    });
  }

  async accept(inviteId: number) {
    if (this.isProcessing(inviteId)) return;
    this.setProcessing(inviteId, true);

    try {
      await this._inviteService.accept(inviteId);
      this._snackBar.open('Invitation accepted!', 'Close', { duration: 3000 });
      this.showNotifications.set(false); // Close dropdown
    } catch (e: any) {
      this._snackBar.open(e?.error?.message || 'Failed to accept invitation', 'Close', { duration: 3000 });
    } finally {
      this.setProcessing(inviteId, false);
    }
  }

  async decline(inviteId: number) {
    if (this.isProcessing(inviteId)) return;
    // if (!confirm('Are you sure you want to decline this invitation?')) return; // Optional confirmation

    this.setProcessing(inviteId, true);
    try {
      await this._inviteService.decline(inviteId);
      this._snackBar.open('Invitation declined', 'Close', { duration: 3000 });
    } catch (e: any) {
      this._snackBar.open(e?.error?.message || 'Failed to decline invitation', 'Close', { duration: 3000 });
    } finally {
      this.setProcessing(inviteId, false);
    }
  }

  logout() {
    this._passport.destroy()
    this._router.navigate(['/login'])
  }
}
