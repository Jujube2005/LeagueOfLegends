import { Component, computed, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { InviteService } from '../_services/invite.service';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatCardModule } from '@angular/material/card';
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { Router } from '@angular/router';

@Component({
    selector: 'app-notifications',
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatIconModule, MatListModule, MatCardModule, MatSnackBarModule],
    templateUrl: './notifications.html',
    styleUrls: ['./notifications.scss']
})
export class NotificationsComponent implements OnInit {
    private inviteService = inject(InviteService);
    private snackBar = inject(MatSnackBar);
    private router = inject(Router);

    invites = this.inviteService.invites;
    processing = signal<Set<number>>(new Set());

    async ngOnInit() {
        await this.inviteService.getMyInvites();
    }

    async accept(inviteId: number) {
        if (this.isProcessing(inviteId)) return;
        this.setProcessing(inviteId, true);

        try {
            await this.inviteService.accept(inviteId);
            this.snackBar.open('Invitation accepted!', 'Close', { duration: 3000 });
            // Optionally navigate to the mission? Let's just update the list for now.
        } catch (e: any) {
            this.snackBar.open(e?.error?.message || 'Failed to accept invitation', 'Close', { duration: 3000 });
        } finally {
            this.setProcessing(inviteId, false);
        }
    }

    async decline(inviteId: number) {
        if (this.isProcessing(inviteId)) return;
        if (!confirm('Are you sure you want to decline this invitation?')) return; // Check only for decline

        this.setProcessing(inviteId, true);
        try {
            await this.inviteService.decline(inviteId);
            this.snackBar.open('Invitation declined', 'Close', { duration: 3000 });
        } catch (e: any) {
            this.snackBar.open(e?.error?.message || 'Failed to decline invitation', 'Close', { duration: 3000 });
        } finally {
            this.setProcessing(inviteId, false);
        }
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
}
