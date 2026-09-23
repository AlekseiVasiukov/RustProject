import { Injectable, inject } from '@angular/core';
import { signal, computed } from '@angular/core';
import { GameService } from './game.service';
import type { GameResult, UnitTypeSelection } from '../models/game.model';

@Injectable({
  providedIn: 'root',
})
export class GameSessionService {
  private gameService = inject(GameService);

  //Unique game instance ID. null inittially.
  gameInstanceId = signal<string | null>(null);

  //list of available faction names
  availableFactions = signal<string[]>([]);

  availableUnitTypes = signal<{ [key: number]: string[] }>({});

  gameResult = signal<GameResult | null>(null);

  //Selection made by user

  ourFactionIndex = signal<number>(0);
  opponentFactionIndex = signal<number>(0);
  ourWarriorTypes = signal<UnitTypeSelection>([0, null, null, null]);

  hasGameStarted = computed(() => this.gameInstanceId !== null);

  hasGameResult = computed(() => this.gameResult() !== null);

  //sart a game, retrieve game_instance_id, factions and unittypes in parallel
  startNewGame() {
    this.gameResult.set(null);
    this.ourFactionIndex.set(0);
    this.opponentFactionIndex.set(0);
    this.ourWarriorTypes.set([0, null, null, null]);

    this.gameService.startNewGame().subscribe((response) => {
      const gameId = response.game_id;
      this.gameInstanceId.set(gameId);

      this.gameService.getFactions(gameId).subscribe((res) => {
        this.availableFactions.set(res.factions);
      });

      this.gameService.getUnittypes(gameId).subscribe((res) => {
        this.availableUnitTypes.set(res.unittypes);
      });
    });
  }

  confirmSelection() {
    const gameId = this.gameInstanceId();
    if (!gameId) return;

    const request = {
      our_faction_index: this.ourFactionIndex(),
      opponent_faction_index: this.opponentFactionIndex(),
      our_unittype_indices: this.ourWarriorTypes(),
    };

    this.gameService.getGameResult(gameId, request).subscribe((response) => {
      this.gameResult.set(response.result);
    });
  }

  isValidSelection(): boolean {
    const warriors = this.ourWarriorTypes();
    return warriors.some((type) => type !== null);
  }
}
