import { Component, Input, OnInit, inject, OnChanges, ChangeDetectorRef, SimpleChanges, OnDestroy } from '@angular/core'
import { CommonModule } from '@angular/common'
import { FormsModule } from '@angular/forms'
import { MissionService } from '../../_services/mission-service'
import { MissionMessage } from '../../_models/mission-message'
import { PassportService } from '../../_services/passport-service'
import { MissionSocketService } from '../../_services/mission-socket.service'
import { Subscription } from 'rxjs'
import { MatIcon } from "@angular/material/icon";

@Component({
  selector: 'app-mission-chat',
  standalone: true,
  imports: [CommonModule, FormsModule, MatIcon],
  templateUrl: './mission-chat.html',
  styleUrls: ['./mission-chat.scss']
})
export class MissionChatComponent implements OnInit, OnChanges, OnDestroy {
  @Input() missionId!: number
  @Input() chiefId?: number

  missionService = inject(MissionService)
  missionSocket = inject(MissionSocketService)
  passport = inject(PassportService)
  cdr = inject(ChangeDetectorRef)

  messages: MissionMessage[] = []
  newMessage: string = ''
  loadingMessages = false
  sendingMessage = false
  filter: 'all' | 'chat' | 'activity' = 'all'

  get filteredMessages() {
    if (this.filter === 'all') return this.messages
    if (this.filter === 'activity') return this.messages.filter(m => m.type_ === 'system')
    return this.messages.filter(m => m.type_ === this.filter)
  }

  private socketSub?: Subscription

  async ngOnInit() {
    // Initial load
    if (this.missionId) {
      this.missionSocket.connect(this.missionId)
      await this.loadMessages()
    }

    this.socketSub = this.missionSocket.messages$.subscribe((msg: MissionMessage) => {
      this.messages.push(msg)
      this.cdr.detectChanges()
      this.scrollToBottom()
    })
  }

  ngOnDestroy() {
    this.missionSocket.disconnect()
    if (this.socketSub) {
      this.socketSub.unsubscribe()
    }
  }

  // Reload when missionId changes (e.g. navigation)
  async ngOnChanges(changes: SimpleChanges) {
    if (changes['missionId'] && !changes['missionId'].firstChange) {
      if (this.missionId) {
        this.missionSocket.connect(this.missionId)
        await this.loadMessages()
      }
    }
  }

  async loadMessages() {
    try {
      this.loadingMessages = true
      this.cdr.detectChanges() // Trigger update for loading spinner

      this.messages = await this.missionService.getMessages(this.missionId)
      // Fix potential timezone issue
      this.messages.forEach(m => {
        if (m.created_at && !m.created_at.endsWith('Z')) {
          m.created_at = m.created_at + 'Z'
        }
      })
      this.scrollToBottom()
    } catch (e) {
      console.error('Failed to load messages', e)
    } finally {
      this.loadingMessages = false
      this.cdr.detectChanges()
    }
  }

  async sendMessage() {
    if (!this.newMessage.trim()) return

    const content = this.newMessage
    this.newMessage = '' // Clear input immediately

    // We send via WebSocket. The response will come back via the subscription.
    this.missionSocket.sendMessage(content)
  }

  private scrollToBottom() {
    setTimeout(() => {
      const list = document.querySelector('.messages-list')
      if (list) {
        list.scrollTop = list.scrollHeight
      }
    }, 100)
  }
}
