import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

import type { Game } from "./util";

export const history: Writable<[] | Game[]> = writable([]);

export function set_history(games: Game[]) {
  history.set(games);
}

export async function update_history(game: Game) {
  history.update((prev) => [game, ...prev]);

  await invoke("insert_game", { data: game });
}
