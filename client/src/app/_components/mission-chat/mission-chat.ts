import { Component, Input, OnInit, inject } from '@angular/core'
import { CommonModule } from '@angular/common'
import { FormsModule } from '@angular/forms'
import { MissionService } from '../../_services/mission-service'
import { MissionMessage } from '../../_models/mission-message'

@Component({
    selector: 'app-mission-chat',
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: './mission-chat.html',
    styles: [`
    .chat-container {
      display: flex;
      flex-direction: column;
      height: 400px;
      border: 1px solid #333;
      border-radius: 8px;
      overflow: hidden;
      background-color: #1e1e1e;
    }
    .messages-list {
      flex: 1;
      overflow-y: auto;
      padding: 10px;
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .message {
      padding: 8px 12px;
      border-radius: 12px;
      max-width: 80%;
      color: #e0e0e0;
    }
    .message.chat {
      background-color: #2c3e50;
      align-self: flex-start;
    }
    .message.system {
      align-self: center;
      background-color: transparent;
      color: #888;
      font-style: italic;
      font-size: 0.9em;
    }
    .input-area {
      display: flex;
      padding: 10px;
      background-color: #252526;
      border-top: 1px solid #333;
    }
    input {
      flex: 1;
      padding: 8px;
      border-radius: 4px;
      border: 1px solid #444;
      background-color: #333;
      color: white;
      margin-right: 10px;
    }
    button {
      padding: 8px 16px;
      background-color: #007acc;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
    button:disabled {
      background-color: #555;
      cursor: not-allowed;
    }
    .sender-name {
        font-size: 0.75em;
        color: #aaa;
        margin-bottom: 2px;
    }
    .msg-content {
        word-wrap: break-word;
    }
    .timestamp {
        font-size: 0.7em;
        color: #666;
        text-align: right;
        margin-top: 4px;
    }
  `]
})
export class MissionChatComponent implements OnInit {
    @Input() missionId!: number

    missionService = inject(MissionService)
    messages: MissionMessage[] = []
    newMessage: string = ''
    loading = false

    async ngOnInit() {
        if (this.missionId) {
            this.loadMessages()
        }
    }

    async loadMessages() {
        try {
            this.loading = true
            this.messages = await this.missionService.getMessages(this.missionId)
        } catch (e) {
            console.error('Failed to load messages', e)
        } finally {
            this.loading = false
        }
    }

    async sendMessage() {
        if (!this.newMessage.trim()) return

        try {
            const msg = this.newMessage
            this.newMessage = ''
            await this.missionService.sendMessage(this.missionId, msg)
            await this.loadMessages()
            // Scroll to bottom?
        } catch (e) {
            console.error('Failed to send message', e)
        }
    }
}
