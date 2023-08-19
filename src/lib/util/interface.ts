export interface Tiles {
  kill: number;
  timestamp: string | null;
  bot: string;
  weapon: string;
  ttk: number;
  shots: number;
  accuracy: number;
  damage_done: number;
  damage_taken: number;
  efficiency: number;
  cheated: boolean | null;
}

export interface KeyValue {
  key: string;
  value: any;
}

export interface Stats {
  weapon: string;
  shots: number;
  hits: number;
  damage_done: number;
  damage_taken: number;
}

export interface Game {
  tiles: [] | Tiles[];
  key_value: [] | KeyValue[];
  stats: Stats;
}

export interface Payload {
  message: string;
  data: Game;
}

export interface MessagePayload {
  message: string;
  data: string;
}

export interface Notification {
  id: string;
  message: string;
  type: string;
  timeout: number;
}

export interface Data {
  tiles: string[];
  key_value: string[];
  stats: string;
}

export interface Scenario {
  id: number;
  name: string;
  difficulty: string;
  created_at: string;
  games_count: number;
}
