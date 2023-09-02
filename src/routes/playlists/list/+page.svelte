<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import type { GroupedPlaylist } from "$lib/util";
  import Playlist from "$lib/Playlist.svelte";
  import Loader from "$lib/Loader.svelte";

  async function fetchScenariosWithData() {
    const data = await invoke<GroupedPlaylist[]>("fetch_playlist_with_data");

    return data;
  }

  const fetchedPlaylists = fetchScenariosWithData();
</script>

<div
  class="w-full h-full p-10 grid grid-cols-2 gap-3 justify-center items-start overflow-y-scroll no-scrollbar no-scrollbar"
>
  {#await fetchedPlaylists}
    <Loader />
  {:then playlists}
    {#each playlists as playlist}
      <Playlist {playlist} />
    {/each}
  {/await}
</div>
