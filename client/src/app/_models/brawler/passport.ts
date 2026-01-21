//DTO สำหรับการรับส่งข้อมูลของ Brawler Passport

export interface Passport {
    access_token: string,
    display_name: string,
    avatar_url?: string,
}

export interface RegisterModel {
    username: string,
    password: string,
    display_name: string,
}

export interface LoginModel {
    username: string,
    password: string,
}