import { Component, Inject, OnInit, ChangeDetectorRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { UserService } from '../../_services/user-service';
import { InviteService } from '../../_services/invite.service';
import { Brawler } from '../../_models/brawler';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatListModule } from '@angular/material/list';
import { MatIconModule } from '@angular/material/icon';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-invite-member',
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatInputModule, MatListModule, MatIconModule, MatDialogModule],
    templateUrl: './invite-member.html',
    styleUrls: ['./invite-member.scss']
})
export class InviteMemberComponent implements OnInit {
    users: Brawler[] = [];
    filteredUsers: Brawler[] = [];
    searchTerm: string = '';
    invitedUsers = new Set<number>();
    missionId: number;
    currentMembers: Set<number>;

    constructor(
        private userService: UserService,
        private inviteService: InviteService,
        public dialogRef: MatDialogRef<InviteMemberComponent>,
        private cdr: ChangeDetectorRef,
        @Inject(MAT_DIALOG_DATA) public data: { missionId: number, currentMembers: number[] }
    ) {
        this.missionId = data.missionId;
        this.currentMembers = new Set(data.currentMembers);
    }

    async ngOnInit() {
        try {
            const allUsers = await this.userService.getAllBrawlers();
            // Filter out current members
            this.users = allUsers.filter(u => !this.currentMembers.has(u.id));
            this.filteredUsers = this.users;
            this.cdr.detectChanges();
        } catch (e) {
            console.error("Failed to load users", e);
        }
    }

    onSearchChange() {
        if (this.searchTerm) {
            const term = this.searchTerm.toLowerCase();
            this.filteredUsers = this.users.filter(u => u.display_name.toLowerCase().includes(term));
        } else {
            this.filteredUsers = this.users;
        }
    }

    async invite(user: Brawler) {
        try {
            await this.inviteService.invite(this.missionId, user.id);
            this.invitedUsers.add(user.id);
        } catch (e) {
            console.error(e);
            alert('Failed to invite user');
        }
    }

    isInvited(userId: number): boolean {
        return this.invitedUsers.has(userId);
    }
}
