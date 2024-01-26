<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { type GroupedPlaylist, PlaylistState } from "$lib/util";
  import Playlist from "$lib/Playlist.svelte";
  import Loader from "$lib/Loader.svelte";
  import PrimaryContainer from "$lib/PrimaryContainer.svelte";

  let selectedPlaylist: GroupedPlaylist | null = null;

  async function fetchScenariosWithData() {
    const data = await invoke<GroupedPlaylist[]>("fetch_playlist_with_data");

    if (data.length > 0) {
      selectedPlaylist = data.filter(
        (playlist) => playlist.state === PlaylistState.ACTIVE
      )[0];

      return data;
    }
    return [];
  }

  const playlists = fetchScenariosWithData();

  $: selectedPlaylist;
</script>

<PrimaryContainer>
  <div class="w-full h-full flex gap-[4rem]">
    <div class="h-full w-[27%] overflow-y-scroll bg-[#CCD4F6]">
      {#await playlists}
        <Loader />
      {:then playlists}
        <ul class="p-2">
          {#each playlists as playlist}
            <li class="mb-2 px-2 flex items-center gap-3">
              <button
                class="text-sm"
                on:click={() => (selectedPlaylist = playlist)}
                >{playlist.name}</button
              >
              {#if playlist.state === PlaylistState.ACTIVE}
                <p
                  class="py-[2px] px-2 rounded-sm text-xs bg-green-600 text-white"
                >
                  active
                </p>
              {/if}
            </li>
          {/each}
        </ul>
      {/await}
    </div>
    <div class="h-full w-[63%] bg-[#CCD4F6] overflow-y-scroll">
      {#if selectedPlaylist !== null}
        <Playlist bind:playlist={selectedPlaylist} />
      {/if}
    </div>
  </div>
</PrimaryContainer>
