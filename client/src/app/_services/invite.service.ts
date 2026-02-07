import { inject, Injectable, signal } from '@angular/core'
import { environment } from '../../environments/environment'
import { HttpClient } from '@angular/common/http'
import { firstValueFrom } from 'rxjs'

@Injectable({
    providedIn: 'root',
})
export class InviteService {
    private _base_url = environment.baseUrl + '/api/mission-invites'
    private _http = inject(HttpClient)
    invites = signal<any[]>([]);

    async invite(missionId: number, userId: number): Promise<any> {
        const url = `${this._base_url}/mission/${missionId}/invite`;
        return await firstValueFrom(this._http.post(url, { user_id: userId }));
    }

    async accept(inviteId: number): Promise<any> {
        const url = `${this._base_url}/invite/${inviteId}/accept`;
        const res = await firstValueFrom(this._http.post(url, {}));
        this.invites.update(prev => prev.filter(i => i.id !== inviteId));
        return res;
    }

    async decline(inviteId: number): Promise<any> {
        const url = `${this._base_url}/invite/${inviteId}/decline`;
        const res = await firstValueFrom(this._http.post(url, {}));
        this.invites.update(prev => prev.filter(i => i.id !== inviteId));
        return res;
    }

    async getMyInvites(): Promise<any[]> {
        const url = `${this._base_url}/my-invites`;
        const data = await firstValueFrom(this._http.get<any[]>(url));
        this.invites.set(data);
        return data;
    }
}
