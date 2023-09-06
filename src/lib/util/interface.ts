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

export interface Settings {
  id: number;
  directory_path?: string;
}

export enum Difficulty {
  EASY = "EASY",
  MEDIUM = "MEDIUM",
  HARD = "HARD",
}

export enum PlaylistState {
  ACTIVE = "ACTIVE",
  INACTIVE = "INACTIVE",
}

export interface GroupedPlaylist {
  id: number;
  name: string;
  description: string;
  duration: number;
  state: PlaylistState;
  scenarios: {
    scenario_name: string;
    scenario_difficulty: Difficulty;
    reps: number;
    days: { games_count: number }[];
  }[];
}

export interface ChartDataSets {
  accuracy: { avg_accuracy: number; date: string }[];
  score: { avg_score: number; date: string }[];
}

export interface ScenarioData {
  id: number;
  name: string;
  games_count: number;
  shots?: number;
  hits?: number;
  accuracy?: number;
  damage_done?: number;
  damage_possible?: number;
  score?: number;
  MaxScore?: number;
  MinScore?: number;
  day_data: ChartDataSets;
  month_data: ChartDataSets;
  year_data: ChartDataSets;
}
