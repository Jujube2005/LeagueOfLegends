export interface Mission {
    id: number
    name: string
    description?: string
    category?: string //*เพิ่ม
    status: string
    chief_id: number
    chief_display_name: string
    crew_count: number
    max_crew: number //*เพิ่ม
    created_at: Date
    updated_at: Date
    is_member?: boolean //*เพิ่ม
    image_url?: string
    difficulty?: string
    duration?: string
    location?: string
    min_level?: number
}