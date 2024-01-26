<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { type GroupedPlaylist, PlaylistState } from "$lib/util";
  import { Button } from "flowbite-svelte";
  import PlaylistScenario from "./PlaylistScenario.svelte";

  export let playlist: GroupedPlaylist;
</script>

<div class="w-full p-3">
  <div class="flex flex-col gap-2 justify-between mb-2">
    <span class="flex items-center gap-2 mb-1">
      <p class="text-3xl">{playlist.name}</p>
      <p class="text-gray-600 text-sm">({playlist.duration} Days)</p>
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
    <p class="text-gray-700 text-sm">{playlist.description}</p>
  </div>
  <div
    class="border-t-black border-t-[1px] pt-2 flex flex-col gap-5 overflow-y-scroll"
  >
    <div class=" space-y-3">
      {#each playlist.scenarios as scenario}
        <PlaylistScenario {scenario} />
      {/each}
    </div>
  </div>
</div>
