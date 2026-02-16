import { ChangeDetectionStrategy, Component, inject, signal } from '@angular/core'
import { MatButtonModule } from '@angular/material/button'
import { MatDialogActions, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog'
import { fileTypeFromBlob } from 'file-type'
import { UserService } from '../../_services/user-service'

@Component({
  selector: 'app-upload-img',
  imports: [MatDialogTitle, MatDialogContent, MatDialogActions, MatButtonModule],
  templateUrl: './upload-img.html',
  styleUrl: './upload-img.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UploadImg {
onFileDropped($event: DragEvent) {
throw new Error('Method not implemented.')
}
  acceptedMimeType = ['image/jpeg', 'image/png', 'image/webp']
  imgFile: File | undefined
  imgPreview = signal<string | undefined>(undefined)
  errorMsg = signal<string | undefined>(undefined) // Local validation errors

  isUploading = signal(false)
  uploadError = signal<string | undefined>(undefined) // Server errors
  isSuccess = signal(false)

  private readonly _dialogRef = inject(MatDialogRef<UploadImg>)
  private readonly _userService = inject(UserService)

  async onSubmit() {
    if (!this.imgFile) return

    this.isUploading.set(true)
    this.uploadError.set(undefined)

    try {
      const error = await this._userService.uploadAvatarImg(this.imgFile)
      if (error) {
        this.uploadError.set(error)
      } else {
        this.isSuccess.set(true)
        // Wait a bit to show success state before reloading
        setTimeout(() => {
          this._dialogRef.close(true)
        }, 1500)
      }
    } catch (e: any) {
      this.uploadError.set(e.message || 'System connectivity failure')
    } finally {
      this.isUploading.set(false)
    }
  }
  async onImgPicked(event: Event) {
    this.imgFile = undefined
    this.imgPreview.set(undefined)
    this.errorMsg.set(undefined)

    const input = event.target as HTMLInputElement
    if (input.files && input.files.length > 0) {
      this.imgFile = input.files[0]
      const fileType = await fileTypeFromBlob(this.imgFile)
      if (fileType && this.acceptedMimeType.includes(fileType.mime)) {
        const reader = new FileReader()
        reader.onerror = () => {
          this.imgFile = undefined
          this.errorMsg.set("some thing went wrong")
        }
        reader.onload = () => {
          this.imgPreview.set(reader.result as string)
        }
        reader.readAsDataURL(this.imgFile)
      } else {
        this.imgFile = undefined
        this.errorMsg.set("image file must be .jpg or .png")
      }
    }
  }
}
