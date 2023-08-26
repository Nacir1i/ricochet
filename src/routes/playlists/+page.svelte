<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import type { GroupedPlaylist } from "$lib/util";
  import Playlist from "$lib/Playlist.svelte";
  import { Input, Textarea, Select, Button } from "flowbite-svelte";
  import { scenarios } from "$lib";

  let unique = {};
  const textAreaProps = {
    id: "description",
    name: "description",
    label: "description",
    rows: 4,
    placeholder: "Description about the playlist",
  };
  const selectScenario = $scenarios.map((scenario) => ({
    value: scenario.id,
    name: scenario.name,
  }));

  async function fetchScenariosWithData() {
    const data = await invoke<GroupedPlaylist[]>("fetch_playlist_with_data");

    return data;
  }

  function restart() {
    unique = {};
  }

  const playlists = fetchScenariosWithData();
  let selected: number;
  let name: string;
  let duration: number;
  let description: string;
  let reps: number;
</script>

{#key unique}
  <div class="w-full h-full p-5 overflow-scroll">
    <h1 class="text-center text-xl">Playlists :</h1>
    <div class="mb-6 flex flex-col gap-5">
      <Input
        id="name"
        size="lg"
        placeholder="Playlist name"
        bind:value={name}
      />
      <Input
        id="duration"
        size="lg"
        placeholder="Duration"
        bind:value={duration}
      />
      <Textarea {...textAreaProps} bind:value={description} />
      <Select class="mt-2" items={selectScenario} bind:value={selected} />
      <Input
        id="reps"
        size="lg"
        placeholder="Reps per scenario"
        type="number"
        bind:value={reps}
      />
      <Button
        on:click={async () => {
          const data = {
            name,
            description,
            duration: Number(duration),
            scenarios: [
              {
                scenario_id: selected,
                reps: Number(reps),
              },
            ],
          };

          console.log(data);

          await invoke("insert_playlist", { data });
          restart();
        }}>Save</Button
      >
    </div>
    {#await playlists}
      <p>...Loading</p>
    {:then playlist}
      <div class="w-full flex flex-col gap-3">
        {#each playlist as set}
          <Playlist playlist={set} />
        {/each}
      </div>
    {/await}
  </div>
{/key}
