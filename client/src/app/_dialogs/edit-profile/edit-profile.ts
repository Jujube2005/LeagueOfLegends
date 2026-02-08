import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Passport } from '../../_models/passport';

@Component({
    selector: 'app-edit-profile',
    standalone: true,
    imports: [CommonModule, FormsModule],
    template: `
    <div class="edit-profile-dialog">
      <div class="dialog-header">
        <h2 class="text-xl font-display uppercase tracking-widest text-primary">Protocol Modification</h2>
        <p class="text-[10px] text-gray-500 uppercase tracking-tighter">Update Hunter Identity Credentials</p>
      </div>

      <div class="dialog-body">
        <div class="input-group">
          <label class="text-[9px] uppercase font-black text-gray-400 tracking-widest mb-2 block">Display Name</label>
          <input [(ngModel)]="displayName" placeholder="Enter operator name..." maxlength="20">
        </div>

        <div class="input-group mt-6">
          <label class="text-[9px] uppercase font-black text-gray-400 tracking-widest mb-2 block">Operator Tagline / Bio</label>
          <textarea [(ngModel)]="tagline" placeholder="Enter mission manifesto..." maxlength="100" rows="3"></textarea>
        </div>
      </div>

      <div class="dialog-footer">
        <button (click)="close()" class="cancel-btn">Abort</button>
        <button (click)="save()" class="save-btn" [disabled]="!displayName.trim()">Apply Changes</button>
      </div>
    </div>
  `,
    styles: [`
    .edit-profile-dialog {
      background: #140c1d;
      border: 1px solid rgba(168, 85, 247, 0.3);
      padding: 2rem;
      border-radius: 1.5rem;
      color: white;
      min-width: 350px;
      box-shadow: 0 0 40px rgba(0,0,0,0.8);
    }
    .dialog-header { margin-bottom: 2rem; }
    .input-group input, .input-group textarea {
      width: 100%;
      background: rgba(255,255,255,0.05);
      border: 1px solid rgba(255,255,255,0.1);
      border-radius: 0.75rem;
      padding: 0.75rem 1rem;
      color: white;
      font-size: 13px;
      transition: all 0.3s ease;
      outline: none;
    }
    .input-group input:focus, .input-group textarea:focus {
      border-color: #a855f7;
      background: rgba(168, 85, 247, 0.1);
      box-shadow: 0 0 15px rgba(168, 85, 247, 0.2);
    }
    .dialog-footer {
      display: flex;
      gap: 1rem;
      margin-top: 2rem;
    }
    .cancel-btn, .save-btn {
      flex: 1;
      padding: 0.75rem;
      border-radius: 0.75rem;
      font-size: 10px;
      font-weight: 900;
      text-transform: uppercase;
      letter-spacing: 0.1em;
      transition: all 0.3s ease;
    }
    .cancel-btn {
      background: rgba(255,255,255,0.05);
      border: 1px solid rgba(255,255,255,0.1);
      color: #94a3b8;
    }
    .cancel-btn:hover { background: rgba(255,255,255,0.1); }
    .save-btn {
      background: rgba(168, 85, 247, 0.2);
      border: 1px solid rgba(168, 85, 247, 0.3);
      color: #d8b4fe;
    }
    .save-btn:hover:not(:disabled) {
      background: #a855f7;
      color: white;
      box-shadow: 0 0 20px rgba(168, 85, 247, 0.4);
    }
    .save-btn:disabled { opacity: 0.5; cursor: not_allowed; }
  `]
})
export class EditProfileDialog {
    private _dialogRef = inject(MatDialogRef<EditProfileDialog>);
    private _data = inject<{ displayName: string, tagline: string }>(MAT_DIALOG_DATA);

    displayName = this._data.displayName;
    tagline = this._data.tagline || '';

    close() { this._dialogRef.close(); }
    save() {
        this._dialogRef.close({
            displayName: this.displayName,
            tagline: this.tagline
        });
    }
}
