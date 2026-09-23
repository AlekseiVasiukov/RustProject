//Api wrappers

//Response from /new_game_id
export interface NewGameIdRepsonse {
  game_id: string;
  success: boolean;
  timestamp: string;
  error: string | null;
}

//Response from /games/{id}/factions
export interface FactionResponse {
  factions: string[];
  success: boolean;
  timestamp: string;
  error: string | null;
}

//Response from /games/{id}/unittypes
export interface UnitTypesResponse {
  unittypes: { [key: number]: string[] };
  success: boolean;
  timestamp: string;
  error: string | null;
}

//Request body for POST /games/{id}/result
export interface GameResultRequest {
  our_faction_index: number;
  opponent_faction_index: number;
  our_unittype_indices: (number | null)[];
}

//Response from POST /games/{id}/result
export interface GameResultApiResponse {
  result: GameResult;
  success: boolean;
  timestamp: string;
  error: string | null;
}

//Game models

//Generated combatant (with points distributed)
export interface Combatant {
  id: string;
  unit_type_name: string;
  unit_type_pretty_name: string | null;
  first_name: string | null;
  last_name: string | null;
  parameters: {
    health: number;
    attack: number;
    range: number;
    defense: number;
  };
  health_points: number;
  current_health_points: number;
}

export interface BattleJournalEvent {
  round: number;
  turn: number;
  cmd: BattleCommand;
}

//command for battle journal
export interface BattleCommand {
  type: 'Attack';
  src: string;
  dst: string;
  dmg: number;
}

export interface BattleJournal {
  winner_side: number; // 0 - our side, 1 - theirs
  events: BattleJournalEvent[];
}

export interface GameResult {
  generated_combatants: { [side: number]: Combatant[] };
  battle_journal: BattleJournal;
}

//Selection of types for our unit
export type UnitTypeSelection = (number | null)[];
