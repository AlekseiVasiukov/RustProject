import { Routes } from '@angular/router';
import { UnitList } from './components/unit-list/unit-list';
import { GameResult } from './components/game-result/game-result';

export const routes: Routes = [
  { path: 'units', component: UnitList },
  { path: 'result', component: GameResult },
  { path: '', redirectTo: '/units', pathMatch: 'full' }
];
