export interface MissionFilter {
    name?: string
    status?: MissionStatus
    category?: string //*เพิ่ม
}

export type MissionStatus =
    'Open' |
    'InProgress' |
    'Completed' |
    'Failed'