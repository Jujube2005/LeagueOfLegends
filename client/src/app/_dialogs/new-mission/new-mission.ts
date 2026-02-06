import { Component, inject } from '@angular/core'
import { AddMission } from '../../_models/add-mission'
import { MatDialogActions, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog'
import { MatButtonModule } from '@angular/material/button'
import { FormsModule } from '@angular/forms'

import { MAT_DIALOG_DATA } from '@angular/material/dialog'

@Component({
  selector: 'app-new-mission',
  imports: [MatDialogTitle, MatDialogContent, MatDialogActions, MatButtonModule, FormsModule],
  templateUrl: './new-mission.html',
  styleUrl: './new-mission.scss',
})
export class NewMission {
  private readonly _data = inject<AddMission>(MAT_DIALOG_DATA, { optional: true })

  addMission: AddMission = {
    name: this._data?.name || '',
    description: this._data?.description || '',
    category: this._data?.category || '',
    max_crew: this._data?.max_crew || 5
  }
  private readonly _dialogRef = inject(MatDialogRef<NewMission>)

  onSubmit() {
    const mission = this.clean(this.addMission)
    this._dialogRef.close(mission)
  }

  private clean(addMission: AddMission): AddMission {
    let max_crew = addMission.max_crew || 5;
    if (max_crew < 2) max_crew = 2;
    if (max_crew > 10) max_crew = 10;

    return {
      name: addMission.name.trim() || 'untitled',
      description: addMission.description?.trim() || undefined,
      category: addMission.category?.trim() || undefined,
      max_crew
    }
  }
}

