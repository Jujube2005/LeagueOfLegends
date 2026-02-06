export interface MissionMessage {
    id: number
    mission_id: number
    user_id?: number
    user_display_name?: string
    user_avatar_url?: string
    content: string
    type_: string // 'chat' | 'system'
    created_at: string
}
