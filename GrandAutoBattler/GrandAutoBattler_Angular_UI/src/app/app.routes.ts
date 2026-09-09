import { Routes } from '@angular/router';
import { UnitList } from './components/unit-list/unit-list';
import { GameResult } from './components/game-result/game-result';
import { NewGame } from './components/new-game/new-game';
export const routes: Routes = [
  { path: 'new_game', component: NewGame },
  { path: 'units_selection', component: UnitList },
  { path: 'result', component: GameResult },
  { path: '', redirectTo: '/units', pathMatch: 'full' }
];
