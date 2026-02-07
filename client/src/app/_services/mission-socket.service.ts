import { Injectable, inject, NgZone } from '@angular/core';
import { Subject } from 'rxjs';
import { environment } from '../../environments/environment';
import { PassportService } from './passport-service';
import { MissionMessage } from '../_models/mission-message';

@Injectable({ providedIn: 'root' })
export class MissionSocketService {
    private socket: WebSocket | null = null;
    private messageSubject = new Subject<MissionMessage>();
    public messages$ = this.messageSubject.asObservable();

    private passport = inject(PassportService);
    private zone = inject(NgZone);

    connect(missionId: number) {
        if (this.socket) {
            this.socket.close();
        }

        const token = this.passport.data()?.token;
        if (!token) {
            console.error("No token found, cannot connect to Mission WS");
            return;
        }

        let wsBaseUrl = environment.baseUrl;
        if (!wsBaseUrl) {
            const protocol = window.location.protocol.replace('http', 'ws');
            wsBaseUrl = `${protocol}//${window.location.host}`;
        } else {
            wsBaseUrl = wsBaseUrl.replace('http', 'ws');
        }

        // Ensure we don't have double slashes if baseUrl has trailing slash
        if (wsBaseUrl.endsWith('/')) wsBaseUrl = wsBaseUrl.slice(0, -1);

        const url = `${wsBaseUrl}/api/ws/mission/${missionId}?token=${token}`;

        this.socket = new WebSocket(url);

        this.socket.onopen = () => {
            console.log(`Connected to Mission ${missionId} Chat Room`);
        };

        this.socket.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                const msg: MissionMessage = {
                    id: -Date.now(), // Temp ID
                    mission_id: missionId,
                    user_id: data.user_id,
                    // Note: Server currently doesn't send display_name/avatar
                    user_display_name: data.user_id === this.passport.userId() ? this.passport.data()?.display_name : undefined,
                    user_avatar_url: data.user_id === this.passport.userId() ? this.passport.data()?.avatar_url : undefined,
                    content: data.content,
                    type_: data.type,
                    created_at: data.created_at
                };
                this.zone.run(() => {
                    this.messageSubject.next(msg);
                });
            } catch (e) {
                console.error("Error parsing WS message", e);
            }
        };

        this.socket.onerror = (err) => {
            console.error('WebSocket error', err);
        };

        this.socket.onclose = () => {
            // console.log('WebSocket closed');
        };
    }

    sendMessage(content: string) {
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(content);
        } else {
            console.warn("WebSocket not connected");
        }
    }

    disconnect() {
        if (this.socket) {
            this.socket.close();
            this.socket = null;
        }
    }
}
