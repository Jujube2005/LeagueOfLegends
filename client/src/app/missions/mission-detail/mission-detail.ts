import { Component, inject, signal, OnInit, computed } from '@angular/core'
import { CommonModule, DatePipe } from '@angular/common'
import { ActivatedRoute, Router, RouterLink } from '@angular/router'
import { MissionService } from '../../_services/mission-service'
import { PassportService } from '../../_services/passport-service'
import { Mission } from '../../_models/mission'
import { ThreeDTiltDirective } from '../../_directives/three-d-tilt.directive'

@Component({
    selector: 'app-mission-detail',
    standalone: true,
    imports: [CommonModule, RouterLink, DatePipe, ThreeDTiltDirective],
    templateUrl: './mission-detail.html',
    styleUrls: ['./mission-detail.scss']
})
export class MissionDetail implements OnInit {
    private _route = inject(ActivatedRoute)
    private _router = inject(Router)
    private _missionService = inject(MissionService)
    public _passport = inject(PassportService)

    mission = signal<Mission | undefined>(undefined)
    crew = signal<any[]>([])
    isLoading = signal<boolean>(true)
    error = signal<string | null>(null)

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
        console.log('MissionDetail OnInit initialized');
        this._route.params.subscribe(params => {
            console.log('Route params changed:', params);
            if (params['id']) {
                this.missionId = +params['id']
                if (!isNaN(this.missionId) && this.missionId > 0) {
                    this.loadMissionData()
                } else {
                    this.error.set("Invalid Mission ID format");
                    this.isLoading.set(false);
                }
            } else {
                this.error.set("No Mission ID provided");
                this.isLoading.set(false);
            }
        })
    }

    async loadMissionData() {
        this.isLoading.set(true)
        this.error.set(null)
        console.log('Loading mission data for ID:', this.missionId);

        try {
            // Fetch mission details
            const mission = await this._missionService.getMission(this.missionId).catch(err => {
                console.error("Failed to fetch mission:", err);
                throw err;
            });

            if (!mission) throw new Error("Mission data is empty");

            // Fetch crew details could fail without blocking the page
            let crew = [];
            try {
                crew = await this._missionService.getCrew(this.missionId);
            } catch (err) {
                console.warn("Failed to fetch crew (non-critical):", err);
            }

            console.log('Mission loaded:', mission);
            console.log('Crew loaded:', crew);

            // Assign random image if not present (UI enhancement)
            const missionAny = mission as any
            const missionWithImg = {
                ...mission,
                image_url: missionAny.image_url || missionAny.imageUrl || `/assets/card/card-img-0${(mission.id % 4) + 1}.jpg`
            } as any

            this.mission.set(missionWithImg)
            this.crew.set(crew)
        } catch (e: any) {
            console.error('Error loading mission data:', e)
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
