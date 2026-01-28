import { Component, computed, inject } from '@angular/core';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service/passport-service';
import { Mission } from '../_models/mission';
import { AsyncPipe, DatePipe } from '@angular/common';
import { BehaviorSubject } from 'rxjs';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatChipsModule } from '@angular/material/chips';
import { FormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatDialog } from '@angular/material/dialog';
import { NewMission } from '../_dialogs/new-mission/new-mission';
import { AddMission } from '../_models/add-mission';

@Component({
  selector: 'app-missions',
  standalone: true,
  imports: [
    AsyncPipe, 
    DatePipe, 
    MatCardModule, 
    MatButtonModule, 
    MatIconModule, 
    MatChipsModule,
    FormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule
  ],
  templateUrl: './missions.html',
  styleUrl: './missions.scss'
})
export class Missions {
  private _missionService = inject(MissionService);
  private _passportService = inject(PassportService);
  private _dialog = inject(MatDialog);
  
  private _missionsSubject = new BehaviorSubject<Mission[]>([]);
  readonly missions$ = this._missionsSubject.asObservable();
  
  isSignin = computed(() => this._passportService['isSignin']);
  
  searchName: string = '';
  searchStatus: string = '';
  filter: { name: string; status: string } = { name: '', status: '' };

  constructor() {
    this.loadMissions();
  }

  async loadMissions() {
    try {
        const filter: any = {};
        if (this.searchName) filter.name = this.searchName;
        if (this.searchStatus) filter.status = this.searchStatus;
        
        const missions = await this._missionService.getMissions(filter);
        this._missionsSubject.next(missions);
    } catch (error) {
        console.error('Error loading missions:', error);
    }
  }
  
  onSubmit() {
    this.searchName = this.filter.name;
    this.searchStatus = this.filter.status;
    this.loadMissions();
  }

  joinMission(mission: Mission) {
    console.log('Join mission:', mission);
    // Future implementation: Call service to join mission
  }

  openDialog() {
    const ref = this._dialog.open(NewMission)
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
        if (!addMission) return;
        
        try {
            await this._missionService.add(addMission);
            // Reload missions to show the new one
            this.loadMissions();
        } catch (error) {
            console.error('Error creating mission:', error);
        }
    })
  }
}
