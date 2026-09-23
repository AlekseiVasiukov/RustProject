import { Routes } from '@angular/router';
import { NewGame } from './components/new-game/new-game';
import { UnitList } from './components/unit-list/unit-list';
import { BattleJournal } from './core/battle-journal/battle-journal';
import { GameResult } from './components/game-result/game-result';
import { Enemies } from './components/enemies/enemies';

export const routes: Routes = [
  { path: 'new-game', component: NewGame },
  { path: 'select-warriors', component: UnitList },
  { path: 'battle-journal', component: BattleJournal },
  { path: 'results', component: GameResult },
  { path: 'enemies', component: Enemies },
  { path: '', redirectTo: '/new-game', pathMatch: 'full' },
];
