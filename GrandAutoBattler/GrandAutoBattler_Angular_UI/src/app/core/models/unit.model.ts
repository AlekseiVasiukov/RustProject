export interface UnitStats {
  h: number;
  a: number;
  r: number;
  d: number;
}

export interface Unit {
  id: string;
  name: string;
  faction: string;
  hp: number;
  stats: UnitStats;
}

export interface UnitTypeResponse {
  [factionIndex: string]: string[];
}
