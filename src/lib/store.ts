import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

import type { Game, Scenario } from "./util";

export const history: Writable<[] | Game[]> = writable([]);
export const dashboardHistory: Writable<[] | Game[]> = writable([]);
export const scenarios: Writable<[] | Scenario[]> = writable([]);

export function set_history(games: Game[]) {
  history.set(games);
}

export async function update_history(game: Game) {
  history.update((prev) => [game, ...prev]);

  await invoke("insert_game", { data: game });
}

export function setDashboardHistory(games: Game[]) {
  dashboardHistory.set(games);
}

export function updateDashboardHistory(game: Game) {
  dashboardHistory.update((prev) => {
    let newData: Game[] = [...prev];

    if (prev.length >= 5) {
      newData.pop();
      newData = [game, ...newData];
    } else {
      newData = [game, ...newData];
    }

    return prev;
  });
}

export function set_scenarios(data: Scenario[]) {
  scenarios.set(data);
}
