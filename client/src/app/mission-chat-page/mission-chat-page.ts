import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { MissionChatComponent } from '../_components/mission-chat/mission-chat';

@Component({
    selector: 'app-mission-chat-page',
    standalone: true,
    imports: [CommonModule, MissionChatComponent, RouterLink],
    template: `
    <div class="p-4" style="max-width: 800px; margin: 0 auto;">
      <div style="margin-bottom: 20px;">
        <a routerLink="/joined-missions" style="color: #007acc; text-decoration: none; cursor: pointer;">&larr; Back to Joined Missions</a>
      </div>
      <h2 style="margin-bottom: 10px; color: #fff;">Mission Chat</h2>
      @if (missionId) {
        <app-mission-chat [missionId]="missionId"></app-mission-chat>
      }
    </div>
  `
})
export class MissionChatPage {
    route = inject(ActivatedRoute);
    missionId?: number;

    constructor() {
        const id = this.route.snapshot.paramMap.get('id');
        if (id) this.missionId = +id;
    }
}
