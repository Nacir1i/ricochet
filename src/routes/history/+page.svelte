<script lang="ts">
  import { history, scenarios } from "$lib";
  import Game from "$lib/Game.svelte";

  $: hidden = true;
</script>

<div
  class="dark:bg-gray-600 relative w-full h-full p-3 flex flex-wrap gap-3 content-start overflow-scroll"
>
  <div class="fixed top-10 right-5 flex flex-col items-end gap-[6px]">
    <button
      id="dropdownDefault"
      class="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-4 py-2.5 text-center inline-flex items-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800"
      type="button"
      on:click={() => (hidden = !hidden)}
    >
      Filter
      <svg
        class="w-4 h-4 ml-2"
        aria-hidden="true"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M19 9l-7 7-7-7"
        />
      </svg>
    </button>

    <div
      id="dropdown"
      class={hidden
        ? "z-10 hidden w-64 max-h-[25rem] p-3 bg-white rounded-lg shadow dark:bg-gray-700 overflow-scroll"
        : "z-10 w-64 max-h-[25rem] p-3 bg-white rounded-lg shadow dark:bg-gray-700 overflow-scroll"}
    >
      <h6 class="mb-3 text-sm font-medium text-gray-900 dark:text-white">
        Scenarios :
      </h6>
      <ul class="space-y-2 text-sm" aria-labelledby="dropdownDefault">
        {#each $scenarios as scenario}
          <li class="flex items-center">
            <input
              id="apple"
              type="checkbox"
              value=""
              class="w-4 h-4 bg-gray-100 border-gray-300 rounded text-primary-600 focus:ring-primary-500 dark:focus:ring-primary-600 dark:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
            />

            <label
              for="apple"
              class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-100"
            >
              {scenario.name} ({scenario.games_count})
            </label>
          </li>
        {/each}
      </ul>
    </div>
  </div>
  {#each $history as game}
    <Game {game} />
  {/each}
</div>
