import { Injectable, NgZone, inject, effect } from '@angular/core';
import { Subject } from 'rxjs';
import { environment } from '../../environments/environment';
import { PassportService } from './passport-service';

export interface Notification {
  title: string;
  message: string;
  type: string;
  metadata: any;
}

@Injectable({
  providedIn: 'root'
})
export class NotificationService {
  private eventSource: EventSource | undefined;
  private notificationSubject = new Subject<Notification>();
  public notifications$ = this.notificationSubject.asObservable();
  private passportService = inject(PassportService);

  constructor(private zone: NgZone) {
    // Attempt to connect immediately if user is already logged in
    this.connect();

    // Watch for passport changes to connect/disconnect automatically
    effect(() => {
      if (this.passportService.data()) {
        this.connect();
      } else {
        this.disconnect();
      }
    });
  }

  showLocalNotification(notification: Notification) {
    this.zone.run(() => {
      this.notificationSubject.next(notification);
    });
  }
  connect(): void {
    if (this.eventSource) {
      this.eventSource.close();
    }

    if (typeof EventSource !== 'undefined') {
      const token = this.passportService.data()?.token;
      console.log('[NotificationService] Connecting with token:', token ? 'YES' : 'NO');
      if (!token) return;

      const url = `${environment.baseUrl}/api/notifications/events?token=${token}`;
      console.log('[NotificationService] URL:', url);
      this.eventSource = new EventSource(url);

      this.eventSource.onopen = (event) => {
        console.log('[NotificationService] Connection opened');
      };

      this.eventSource.onmessage = (event) => {
        console.log('[NotificationService] Message received:', event.data);
        this.zone.run(() => {
          try {
            const data = JSON.parse(event.data);
            const notification: Notification = {
              title: data.title,
              message: data.message,
              type: data.notification_type,
              metadata: data.metadata
            };
            console.log('[NotificationService] Emitting notification:', notification);
            this.notificationSubject.next(notification);
          } catch (e) {
            console.error('[NotificationService] Error parsing notification', e);
          }
        });
      };

      this.eventSource.onerror = (error) => {
        console.error('[NotificationService] EventSource error:', error);
        if (this.eventSource?.readyState === EventSource.CLOSED) {
          console.log('[NotificationService] Connection closed');
        }
      };
    }
  }

  disconnect() {
    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = undefined;
    }
  }
}
