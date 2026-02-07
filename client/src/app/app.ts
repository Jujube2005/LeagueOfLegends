import { Component, signal, inject, OnInit } from '@angular/core'
import { RouterOutlet } from '@angular/router'
import { Navbar } from "./navbar/navbar"
import { Toast } from "./_components/toast/toast"
import { InviteService } from "./_services/invite.service" // Adjust path if needed
import { NotificationService } from "./_services/notification-service"
import { PassportService } from "./_services/passport-service"

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Navbar, Toast],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App implements OnInit {
  protected readonly title = signal('client');
  private inviteService = inject(InviteService);
  private notificationService = inject(NotificationService);
  private passportService = inject(PassportService);

  async ngOnInit() {
    if (this.passportService.data()) {
      this.checkInvites();
    }
  }

  async checkInvites() {
    try {
      // Add small delay to ensure everything is ready
      setTimeout(async () => {
        if (!this.passportService.data()) return;
        const invites = await this.inviteService.getMyInvites();
        if (invites && invites.length > 0) {
          this.notificationService.showLocalNotification({
            title: 'Mission Invites',
            message: `You have ${invites.length} pending invite(s). Check your profile!`,
            type: 'info',
            metadata: {}
          });
        }
      }, 2000);
    } catch (e) {
      console.error("Failed to check invites", e);
    }
  }
}
