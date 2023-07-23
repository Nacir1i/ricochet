<script lang="ts">
  import type { Game } from "./util";

  export let game: Game;
</script>

<div
  class="w-[25rem] h-36 flex dark:bg-slate-600 bg-slate-400 flex-col justify-center items-center rounded-md text-black dark:text-white"
>
  <div class="w-full p-1 flex flex-col justify-center items-center">
    <h1>{game.key_value[3].value}</h1>
    <span class="flex gap-2">
      <p class="text-green-700 dark:text-green-400">Score:</p>
      <p>{game.key_value[2].value}</p>
    </span>
  </div>
  <div
    class="w-full h-full bg-gray-200 dark:bg-gray-700 rounded-br-md rounded-bl-md"
  >
    <ul
      class="w-full grid grid-flow-col grid-cols-3 grid-rows-2 flex-wrap gap-3 p-3 text-sm"
    >
      <li class="flex flex-wrap">
        <p>Kills:</p>
        <p>{game.key_value[0].value}</p>
      </li>
      <li class="flex flex-wrap">
        <p>Accuracy:</p>
        <p>{Math.floor((game.stats.hits / game.stats.shots) * 100)}%</p>
      </li>
      <li class="flex flex-wrap">
        <p>Shots taken:</p>
        <p>{game.stats.shots}</p>
      </li>
      <li class="flex flex-wrap">
        <p>Shots hit:</p>
        <p>{game.stats.hits}</p>
      </li>
      <li class="flex flex-wrap">
        <p>Sens:</p>
        <p>XX</p>
      </li>
      <li class="flex flex-wrap">
        <p>FOV:</p>
        <p>XXX</p>
      </li>
    </ul>
  </div>
</div>
