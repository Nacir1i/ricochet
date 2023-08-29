<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { type GroupedPlaylist, PlaylistState } from "$lib/util";
  import { Button } from "flowbite-svelte";
  import { ChevronDown, ChevronUp } from "lucide-svelte";

  export let playlist: GroupedPlaylist;

  let toggle = false;
</script>

<div
  data-te-perfect-scrollbar-init
  class="w-[500px] max-w-[500px] dark:bg-gray-700 bg-gray-200 p-3 dark:text-white rounded-lg"
>
  <div class="flex items-center justify-between mb-2">
    <div class="flex flex-col gap-2">
      <span class="flex items-center gap-2 mb-1">
        <p class="text-lg">Name:</p>
        <p>{playlist.name}</p>
        <Button
          class="py-[2px] px-2 rounded-sm text-xs"
          color={playlist.state === PlaylistState.ACTIVE ? "green" : "red"}
          on:click={() =>
            invoke("update_playlist_state", {
              playlistId: playlist.id,
              state:
                playlist.state === PlaylistState.ACTIVE
                  ? PlaylistState.INACTIVE
                  : PlaylistState.ACTIVE,
            })}>{playlist.state}</Button
        >
      </span>
      <span class="flex items-center gap-2">
        <p class="text-lg">Description:</p>
        <p>{playlist.description}</p>
      </span>
      <span class="flex items-center gap-2">
        <p class="text-lg">Duration:</p>
        <p>{playlist.duration}</p>
      </span>
    </div>

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
  </div>
  {#if toggle}
    <div
      class="border-t-2 dark:border-white border-gray-400 pt-2 flex flex-col gap-5"
    >
      {#each playlist.scenarios as scenario}
        <div>
          <span class="flex items-center gap-2">
            <p>Scenario name :</p>
            <p>{scenario.scenario_name}</p>
          </span>
          <span class="flex items-center gap-2">
            <p>Scenario difficulty :</p>
            <p>{scenario.scenario_difficulty}</p>
          </span>
          <span class="flex items-center gap-2">
            <p>Repititions :</p>
            <p>{scenario.reps}</p>
          </span>
          <span>
            <p>Games played per day :</p>
            <ul class="ml-3 pl-5">
              {#each scenario.days as day, index}
                <span class="flex items-center gap-2">
                  <p>Day {index + 1}:</p>
                  <p>{day.games_count}</p>
                </span>
              {/each}
            </ul>
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>
