<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import type { GroupedPlaylist, ScenarioData } from "$lib/util";
  import Loader from "$lib/Loader.svelte";
  import SmallPrimaryContainer from "$lib/SmallPrimaryContainer.svelte";
  import ScenarioChart from "$lib/ScenarioChart.svelte";
  import ProgressBar from "@okrad/svelte-progressbar";

  async function fetchActivePlaylist() {
    const data = await invoke<GroupedPlaylist[]>(
      "fetch_active_playlist_with_data"
    );

    let gamesToPlay = 0;
    let playedGames = 0;

    data[0]?.scenarios.forEach((scenario) => {
      gamesToPlay += scenario.reps;

      scenario.days.forEach((day) => {
        playedGames += day.games_count;
      });
    });
    gamesToPlay = gamesToPlay * data[0]?.duration;

    return { data, gamesToPlay, playedGames };
  }

  async function fetch_scenarios(): Promise<ScenarioData[]> {
    const data = await invoke<ScenarioData[]>("fetch_scenario_data");

    return data;
  }

  const scenarios = fetch_scenarios();
  const activePlaylist = fetchActivePlaylist();

  //This disables contextmenu, (should prob find a better solution)
  // document.addEventListener("contextmenu", (event) => event.preventDefault());
</script>

<div class="w-full h-full flex flex-col gap-6">
  <div class="w-full h-[60%] bg-green-300">
    <SmallPrimaryContainer>
      <!-- {#await scenarios}
        <Loader />
      {:then scenarios}
        <div
          class="w-ful h-full flex justify-start items-center gap-7 overflow-y-scroll scrollbar-thin scrollbar-thumb-third"
        >
          {#each scenarios as scenario}
            <ScenarioChart {scenario} />
          {/each}
        </div>
      {/await} -->
    </SmallPrimaryContainer>
  </div>
  <div class="w-full flex-1 z-50 flex justify-between items-center gap-6">
    <div
      class="p-5 z-50 w-[50%] max-w-[50%] h-full max-h-full bg-primary border-accent border-4 relative"
    >
      <!-- {#await activePlaylist}
        <Loader />
      {:then activePlaylist}
        <div class="flex flex-col overflow-y-scroll max-h-[150px]">
          <div class="flex gap-3 items-center">
            <p class="text-white text-xl">{activePlaylist.data[0].name}</p>
            <ProgressBar
              labelColor="white"
              thickness={6}
              series={{
                perc:
                  (activePlaylist.playedGames / activePlaylist.gamesToPlay) *
                    100 <=
                  100
                    ? (activePlaylist.playedGames /
                        activePlaylist.gamesToPlay) *
                      100
                    : 100,
                color: "#2196f3",
              }}
              width={60}
              style="radial"
              textSize={90}
            />
          </div>
          <ul>
            {#each activePlaylist.data[0].scenarios as scenario}
              <li>{scenario.scenario_name}</li>
            {/each}
          </ul>
        </div>
      {/await} -->
      <div
        class="bg-secondary border-accent border-t-4 absolute -bottom-14 -left-[3.6rem] h-14 w-[3.6rem] transform translate-x-1/2 -translate-y-1/2 rotate-45"
      />
    </div>
    <div class="z-40 w-[50%] h-full bg-primary border-accent border-4 relative">
      <div
        class="bg-secondary border-accent border-t-4 absolute -bottom-14 -left-[3.7rem] h-14 w-[3.6rem] transform translate-x-1/2 -translate-y-1/2 rotate-45"
      />
    </div>
  </div>
</div>
