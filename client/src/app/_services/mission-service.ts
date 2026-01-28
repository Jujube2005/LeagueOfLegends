import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { AddMission } from '../_models/add-mission';
import { Mission } from '../_models/mission';
import { firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class MissionService {
  private _http = inject(HttpClient);
  private _api_url = environment.baseUrl;

  async add(mission: AddMission): Promise<number> {
    const url = this._api_url + '/api/mission-management';
    // The backend might return the ID directly as a number or string, not necessarily an object
    const observable = this._http.post<any>(url, mission);
    const resp = await firstValueFrom(observable);
    
    if (resp && typeof resp === 'object' && 'mission_id' in resp) {
        return resp.mission_id;
    }
    
    const id = parseInt(resp, 10);
    if (!isNaN(id)) {
        return id;
    }
    
    throw new Error('Invalid response from add mission');
  }

  async getMyMissions(): Promise<Mission[]> {
    const url = this._api_url + '/api/brawler/my-missions';
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
  }

  async getMissions(filter?: any): Promise<Mission[]> {
    let url = this._api_url + '/api/view/filter';
    if (filter) {
      // Simple query param serialization for now, or use HttpParams
      const params = new URLSearchParams();
      if (filter.name) params.append('name', filter.name);
      if (filter.status) params.append('status', filter.status);
      url += '?' + params.toString();
    }
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
  }
}
