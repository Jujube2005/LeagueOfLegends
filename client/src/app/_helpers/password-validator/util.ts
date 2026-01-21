import { Passport } from "../../_models/brawler/passport"

const _default_avatar =  '/assets/default.avatar.jpg'

export function getavatarUrl(passport: Passport | undefined): string {
    if(passport && passport.avatar_url) return passport.avatar_url
    return _default_avatar
}