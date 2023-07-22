import { writable, type Writable } from "svelte/store";

import type { Game } from "./util";

let history: Writable<[] | Game[]> = writable([]);

export function set_history(games: Game[]) {
  history.set(games);
}

export function update_history(game: Game) {
  history.update((prev) => [game, ...prev]);
}
