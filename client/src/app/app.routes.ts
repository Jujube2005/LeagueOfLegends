import { Routes } from '@angular/router'
import { Home } from './home/home'
import { Login } from './login/login'
import { Profile } from './profile/profile'
import { ServerError } from './server-error/server-error'
import { NotFound } from './not-found/not-found'
import { authGuard } from './_guard/auth-guard'
import { Missions } from './missions/missions'
import { MissionManager } from './missions/mission-manager/mission-manager'
import { JoinedMissions } from './joined-missions/joined-missions'
import { MissionSummaryComponent } from './mission-summary/mission-summary'
import { LeaderboardComponent } from './leaderboard/leaderboard'

export const routes: Routes = [
    { path: '', component: Home },
    { path: 'login', component: Login },
    // *เพิ่ม
    // { path: 'recover-password', loadComponent: () => import('./recover-password/recover-password').then(m => m.RecoverPassword) },
    { path: 'profile', component: Profile, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    { path: 'missions', component: Missions, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    {
        path: 'missions/:id',
        loadComponent: () => import('./missions/mission-detail/mission-detail').then(m => m.MissionDetail),
        canActivate: [authGuard]
    },
    // *้เพิ่ม
    { path: 'joined-missions', component: JoinedMissions, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    // *Chat Page
    {
        path: 'mission-chat/:id',
        loadComponent: () => import('./mission-chat-page/mission-chat-page').then(m => m.MissionChatPage),
        canActivate: [authGuard]
    },
    // *้เพิ่ม
    { path: 'mission-summary', component: MissionSummaryComponent, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    // *้เพิ่ม
    { path: 'leaderboard', component: LeaderboardComponent, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    {
        path: 'notifications',
        loadComponent: () => import('./notifications/notifications').then(m => m.NotificationsComponent),
        canActivate: [authGuard]
    },
    {
        path: 'chief',
        component: MissionManager,
        runGuardsAndResolvers: 'always',
        canActivate: [authGuard]
    },
    // *เพิ่ม
    {
        path: 'chief/mission/:id',
        loadComponent: () => import('./missions/mission-specific-manager/mission-specific-manager').then(m => m.MissionSpecificManager),
        canActivate: [authGuard]
    },

    { path: 'server-error', component: ServerError },
    { path: '**', component: NotFound },
]
