<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import type { GroupedPlaylist } from "$lib/util";
  import Playlist from "$lib/Playlist.svelte";

  async function fetchScenariosWithData() {
    const data = await invoke<GroupedPlaylist[]>("fetch_playlist_with_data");

    return data;
  }

  const playlists = fetchScenariosWithData();
</script>

<div
  class="w-full h-full p-5 flex flex-col gap-3 justify-start items-center overflow-y-scroll no-scrollbar no-scrollbar"
>
  <h1 class="text-center text-xl">Playlists :</h1>
  {#await playlists}
    <p>...Loading</p>
  {:then playlist}
    {#each playlist as set}
      <Playlist playlist={set} />
    {/each}
  {/await}
</div>
