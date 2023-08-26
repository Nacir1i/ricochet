<script lang="ts">
  import type { GroupedPlaylist } from "$lib/util";
  import { ChevronDown, ChevronUp } from "lucide-svelte";

  export let playlist: GroupedPlaylist;

  let toggle = false;
</script>

<div class="bg-red-300 p-3">
  <p>name: {playlist.name}</p>
  <p>description: {playlist.description}</p>
  <p>duration: {playlist.duration}</p>
  <p>duration: {playlist.state}</p>
  <button
    on:click={() => {
      toggle = !toggle;
      console.log("clicked");
    }}
  >
    {#if toggle}
      <ChevronUp class="cursor-pointer" />
    {:else}
      <ChevronDown class="cursor-pointer" />
    {/if}
  </button>
  {#if toggle}
    <div class="bg-green-300">
      {#each playlist.scenarios as scenario}
        <p>{scenario.scenario_name}</p>
        <p>{scenario.scenario_difficulty}</p>
        <p>{scenario.reps}</p>
        Days:
        <ul class="list-disc ml-3 p-5">
          {#each scenario.days as day}
            <li>{day.games_count}</li>
          {/each}
        </ul>
      {/each}
    </div>
  {/if}
</div>
