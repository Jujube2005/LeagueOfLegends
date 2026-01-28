import { Routes } from '@angular/router';
import { Home } from './home/home';
import { Login } from './login/login';
import { NotFound } from './not-found/not-found';
import { ServerError } from './server-error/server-error';
import { Profile } from './profile/profile';
import { MissionManager } from './missions/mission-manager/mission-manager';
import { Missions } from './missions/missions';
import { authGuard } from './_guard/auth-guard';

export const routes: Routes = [
{ path: '', component: Home },
{ path: 'login', component: Login },
{ path: 'profile', component: Profile },
{ path: 'missions', component: Missions },
{
  path: 'chief',
  component: MissionManager,
  runGuardsAndResolvers: 'always',
  canActivate: [authGuard]
},
{ path: 'server-error', component: ServerError },
{ path: '**', component: NotFound },
];