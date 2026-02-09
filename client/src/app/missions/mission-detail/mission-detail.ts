import { Component, inject, signal, OnInit, computed, Input } from '@angular/core'
import { CommonModule, DatePipe } from '@angular/common'
import { ActivatedRoute, Router, RouterLink } from '@angular/router'
import { MissionService } from '../../_services/mission-service'
import { PassportService } from '../../_services/passport-service'
import { Mission } from '../../_models/mission'
import { ThreeDTiltDirective } from '../../_directives/three-d-tilt.directive'
import { MissionChatComponent } from '../../_components/mission-chat/mission-chat'

@Component({
    selector: 'app-mission-detail',
    standalone: true,
    imports: [CommonModule, RouterLink, DatePipe, ThreeDTiltDirective, MissionChatComponent],
    templateUrl: './mission-detail.html',
    styleUrls: ['./mission-detail.scss']
})
export class MissionDetail implements OnInit {
    private _route = inject(ActivatedRoute)
    private _router = inject(Router)
    private _missionService: MissionService = inject(MissionService)
    public _passport: PassportService = inject(PassportService)

    mission = signal<Mission | undefined>(undefined)
    crew = signal<any[]>([])
    isLoading = signal<boolean>(true)
    error = signal<string | null>(null)

    @Input() set id(val: number | string | null) {
        if (val) {
            this.missionId = Number(val);
            this.loadMissionData();
        }
    }

    missionId: number = 0

    // Computed signals for reactive state
    isMember = computed(() => {
        const userId = this._passport.userId()
        if (!userId) return false
        return this.crew().some(member => member.id === userId)
    })

    isChief = computed(() => {
        return this.mission()?.chief_id === this._passport.userId()
    })

    ngOnInit() {
        this._route.params.subscribe(params => {
            if (params['id'] && this.missionId === 0) {
                this.missionId = +params['id']
                if (!isNaN(this.missionId) && this.missionId > 0) {
                    this.loadMissionData()
                }
            }
        })
    }

    async loadMissionData() {
        this.isLoading.set(true)
        this.error.set(null)

        try {
            // Fetch mission details
            const mission = await this._missionService.getMission(this.missionId);

            if (!mission) throw new Error("Mission not found");

            // Fetch crew details - non-critical, continue even if it fails
            let crew: any[] = [];
            try {
                crew = await this._missionService.getCrew(this.missionId);
            } catch (err) {
                console.warn(`Could not load crew for mission ${this.missionId}:`, err);
                // Continue with empty crew array
            }

            // Assign fallback image if not present
            const missionWithImg = {
                ...mission,
                image_url: (mission as any).image_url || (mission as any).imageUrl || `/assets/card/card-img-0${(mission.id % 4) + 1}.jpg`
            };

            this.mission.set(missionWithImg as Mission)
            this.crew.set(crew)
        } catch (e: any) {
            console.error('Failed to load mission:', e)
            this.error.set(e?.message || e?.error?.message || 'Failed to load mission data')
        } finally {
            this.isLoading.set(false)
        }
    }

    async joinMission() {
        if (!this.mission()) return
        try {
            await this._missionService.joinMission(this.missionId)
            await this.loadMissionData() // Reload to update crew list
        } catch (e: any) {
            alert(e?.error?.message || 'Failed to join mission')
        }
    }

    async leaveMission() {
        if (!confirm('Abort mission protocol?')) return
        try {
            await this._missionService.leaveMission(this.missionId)
            await this.loadMissionData() // Reload to update crew list
        } catch (e: any) {
            alert(e?.error?.message || 'Failed to leave mission')
        }
    }
}
