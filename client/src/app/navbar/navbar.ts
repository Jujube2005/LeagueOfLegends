import { Component, computed, inject, Signal, signal, effect, OnDestroy, ChangeDetectorRef } from '@angular/core'
import { PassportService } from '../_services/passport-service'
import { Router, RouterLink, RouterLinkActive } from "@angular/router"
import { InviteService } from '../_services/invite.service'
import { NotificationService } from '../_services/notification-service'
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { CommonModule } from '@angular/common';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { ConfirmationDialogComponent } from '../_dialogs/confirmation-dialog/confirmation-dialog';
import { globalPolling } from '../_services/polling-service';


@Component({
  selector: 'app-navbar',
  imports: [RouterLink, RouterLinkActive, MatSnackBarModule, CommonModule, MatDialogModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',
})
export class Navbar implements OnDestroy {
  private _router = inject(Router)
  private _passport = inject(PassportService)
  private _inviteService = inject(InviteService)
  private _notificationService = inject(NotificationService)
  private _dialog = inject(MatDialog)
  private _snackBar = inject(MatSnackBar)
  private _cdr = inject(ChangeDetectorRef)

  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  invite_count = computed(() => this._inviteService.invites().length)
  isHidden = signal(false)
  showNotifications = signal(false)
  processing = signal<Set<number>>(new Set())
  invites = this._inviteService.invites;
  private _unsubscribePolling?: () => void;


  constructor() {
    this._router.events.subscribe(() => {
      this.isHidden.set(this._router.url.includes('/login') || this._router.url.includes('/register'))
    })
    this.display_name = computed(() => this._passport.data()?.display_name)
    this.avatar_url = computed(() => this._passport.avatar())

    effect(() => {
      if (this.display_name()) {
        this.updateInviteCount()
        this.startPolling();
      } else {
        this.stopPolling();
      }
    })

    this._notificationService.notifications$.subscribe(() => {
      this.updateInviteCount()
    })
  }

  ngOnDestroy() {
    this.stopPolling();
  }

  startPolling() {
    globalPolling.start();
    if (!this._unsubscribePolling) {
      this._unsubscribePolling = globalPolling.subscribe(() => {
        if (this.display_name()) {
          this.updateInviteCount();
        }
      });
    }
  }

  stopPolling() {
    if (this._unsubscribePolling) {
      this._unsubscribePolling();
      this._unsubscribePolling = undefined;
    }
    globalPolling.stop();
  }


  async updateInviteCount() {
    try {
      if (!this.display_name()) return;
      await this._inviteService.getMyInvites();
    } catch (e) {
      // console.error('Failed to fetch invites', e);
    }
  }

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

    const dialogRef = this._dialog.open(ConfirmationDialogComponent, {
      width: '400px',
      panelClass: 'premium-dialog-panel',
      data: {
        title: 'Accept Mission?',
        message: 'You are about to join this mission. Confirm authorization?',
        confirmText: 'Engage',
        cancelText: 'Stand Down'
      }
    });

    dialogRef.afterClosed().subscribe(async (result) => {
      if (result) {
        this.setProcessing(inviteId, true);
        try {
          await this._inviteService.accept(inviteId);
          this._snackBar.open('Invitation accepted!', 'Close', { duration: 3000 });
          this.showNotifications.set(false); // Close dropdown
          this._cdr.detectChanges(); // Ensure UI updates immediately
        } catch (e: any) {
          const msg = typeof e.error === 'string' ? e.error : (e.error?.message || 'Failed to accept invitation');
          this._snackBar.open(msg, 'Close', { duration: 3000 });
        } finally {
          this.setProcessing(inviteId, false);
        }
      }
    });

  }

  async decline(inviteId: number) {
    if (this.isProcessing(inviteId)) return;

    const dialogRef = this._dialog.open(ConfirmationDialogComponent, {
      width: '400px',
      panelClass: 'premium-dialog-panel',
      data: {
        title: 'Decline Invitation?',
        message: 'Reject this mission assignment? This action cannot be undone.',
        confirmText: 'Reject',
        cancelText: 'Cancel'
      }
    });

    dialogRef.afterClosed().subscribe(async (result) => {
      if (result) {
        this.setProcessing(inviteId, true);
        try {
          await this._inviteService.decline(inviteId);
          this._snackBar.open('Invitation declined', 'Close', { duration: 3000 });
          this._cdr.detectChanges(); // Ensure UI updates immediately
        } catch (e: any) {
          const msg = typeof e.error === 'string' ? e.error : (e.error?.message || 'Failed to decline invitation');
          this._snackBar.open(msg, 'Close', { duration: 3000 });
        } finally {
          this.setProcessing(inviteId, false);
        }
      }
    });
  }

  logout() {
    this._passport.destroy()
    this._router.navigate(['/login'])
  }
}
