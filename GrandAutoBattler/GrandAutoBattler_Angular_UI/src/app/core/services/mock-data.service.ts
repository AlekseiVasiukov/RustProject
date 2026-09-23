import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { Unit, UnitTypesResponse } from '../models/model';

@Injectable({
  providedIn: 'root'
})
export class MockDataService {

  constructor() { }

  getGameId(): Observable<string> {
    return of('mock-game-id-123');
  }

  getFactions(): Observable<string[]> {
    return of(['Humans', 'Demons']);
  }

  getUnitTypes(): Observable<UnitTypesResponse> {
    return of({
      '0': ['Knight', 'Archer', 'Witch'],
      '1': ['Succubus', 'Demon', 'Imp']
    });
  }

  getUnits(gameId: string): Observable<Unit[]> {
    // Simulating the data from the example provided in the prompt
    const mockUnits: Unit[] = [
      { id: '1', name: 'Kaelis Nightweaver', faction: 'Humans', hp: 120, stats: { h: 8, a: 4, r: 4, d: 6 } },
      { id: '2', name: 'Jaina Moonshadow', faction: 'Humans', hp: 100, stats: { h: 8, a: 8, r: 9, d: 1 } },
      { id: '3', name: 'Morgath Swiftwind', faction: 'Humans', hp: 90, stats: { h: 8, a: 9, r: 9, d: 1 } },
      { id: '4', name: 'Zerath Shadowstalker', faction: 'Humans', hp: 115, stats: { h: 7, a: 7, r: 6, d: 0 } },
      { id: '5', name: 'Draka the Tormented', faction: 'Demons', hp: 125, stats: { h: 9, a: 9, r: 3, d: 2 } },
      { id: '6', name: 'Belial Bloodweaver', faction: 'Demons', hp: 125, stats: { h: 9, a: 9, r: 3, d: 4 } },
      { id: '7', name: 'Vorg Doomwhisper', faction: 'Demons', hp: 110, stats: { h: 6, a: 9, r: 3, d: 3 } },
      { id: '8', name: 'Azgor Xarath', faction: 'Demons', hp: 115, stats: { h: 7, a: 9, r: 3, d: 9 } },
    ];
    return of(mockUnits);
  }

  getGameResult(gameId: string): Observable<any> {
    return of({
      winner: 'Humans',
      battleJournal: [
        'Battle started!',
        'Kaelis Nightweaver attacks Draka the Tormented for 10 damage.',
        'Draka the Tormented counter-attacks for 5 damage.',
        'Battle finished!'
      ]
    });
  }
}
