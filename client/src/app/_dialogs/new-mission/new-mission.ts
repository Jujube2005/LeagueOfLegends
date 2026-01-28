import { Component, inject } from '@angular/core';
import { AddMission } from '../../_models/add-mission';
import { MatDialogRef, MatDialogContent, MatDialogActions, MatDialogClose, MatDialogTitle } from '@angular/material/dialog';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
  selector: 'app-new-mission',
  standalone: true,
  imports: [
    FormsModule, 
    MatDialogContent, 
    MatDialogActions, 
    MatDialogClose, 
    MatButtonModule, 
    MatInputModule, 
    MatFormFieldModule,
    MatDialogTitle
  ],
  templateUrl: './new-mission.html',
  styleUrl: './new-mission.scss'
})
export class NewMission {
  addMission: AddMission = {
    name: '',
    description: ''
  }
  private readonly _dialogRef = inject(MatDialogRef<NewMission>)

  onSubmit() {
    const mission = this.clean(this.addMission)
    this._dialogRef.close(mission)
  }

  private clean(addMission: AddMission): AddMission {
    return {
      name: addMission.name.trim() || 'untitle',
      description: addMission.description?.trim() || undefined
    }
  }
}
