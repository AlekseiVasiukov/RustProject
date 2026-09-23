import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../../environments/environment';
import {
  NewGameIdRepsonse,
  FactionResponse,
  UnitTypesResponse,
  GameResultRequest,
  GameResultApiResponse,
} from '../models/game.model';

@Injectable({
  providedIn: 'root',
})
export class GameService {
  private http = inject(HttpClient);
  private baseUrl = environment.apiUrl;

  //POST /new_game_id
  startNewGame(): Observable<NewGameIdRepsonse> {
    return this.http.post<NewGameIdRepsonse>(`${this.baseUrl}/new_game_id`, {});
  }

  //GET /games/{gameId}/faction
  getFactions(gameId: string): Observable<FactionResponse> {
    return this.http.get<FactionResponse>(`${this.baseUrl}/games/${gameId}/factions`);
  }

  //Get /games/{gameId}/unittypes
  getUnittypes(gameId: string): Observable<UnitTypesResponse> {
    return this.http.get<UnitTypesResponse>(`${this.baseUrl}/games/${gameId}/unittypes`);
  }

  //POST /games/{gameId}/result
  getGameResult(gameId: string, request: GameResultRequest): Observable<GameResultApiResponse> {
    return this.http.post<GameResultApiResponse>(`${this.baseUrl}/games/${gameId}/result`, request);
  }
}
