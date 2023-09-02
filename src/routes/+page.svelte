<script lang="ts">
  import { dashboardHistory } from "$lib";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api";
  import type {
    ChartStats,
    FormattedChart,
    GenericStats,
    GroupedPlaylist,
    Stat,
  } from "$lib/util";
  import Loader from "$lib/Loader.svelte";
  import GenericStatsComp from "$lib/GenericStatsComp.svelte";

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

  async function fetch_stats(): Promise<Stat[]> {
    const generic: [] | GenericStats[] = await invoke(
      "fetch_general_scenario_stats"
    );
    const charts: [] | ChartStats[] = await invoke(
      "fetch_chart_scenario_stats"
    );

    return formatStats(charts, generic);
  }

  function formatStats(charts: ChartStats[], generics: GenericStats[]) {
    let stats: Stat[] = [];

    for (let i = 0; i < charts.length; i++) {
      let data: FormattedChart;
      let accuracyArray: number[] = [];
      let dateArray: string[] = [];

      for (let j = 0; j < charts[i].data.length; j++) {
        accuracyArray = [...accuracyArray, charts[i].data[j].avg_accuracy ?? 0];
        dateArray = [...dateArray, charts[i].data[j].date];
      }

      data = { name: charts[i].name, accuracyArray, dateArray };
      const correspondingGeneric = generics.filter(
        (generic) => generic.name === charts[i].name
      )[0];

      stats = [...stats, { chart: data, generic: correspondingGeneric }];
    }

    return stats
      .sort((key1, key2) => key2.generic.games_count - key1.generic.games_count)
      .slice(0, 5);
  }

  const stats = fetch_stats();
  const activePlaylist = fetchActivePlaylist();

  //This disables contextmenu, (should prob find a better solution)
  document.addEventListener("contextmenu", (event) => event.preventDefault());
</script>

<div class="w-full h-full p-10 pt-2 flex overflow-y-scroll no-scrollbar gap-16">
  <div
    class="h-full w-full rounded-md flex flex-col overflow-y-scroll no-scrollbar"
  >
    <h1
      class="mb-3 text-xl font-semibold text-gray-900 dark:text-white text-left"
    >
      Top played scenarios
    </h1>
    {#await stats}
      <Loader />
    {:then awaitedStats}
      <div class="h-full w-full flex flex-col gap-11">
        {#each awaitedStats as stat}
          <GenericStatsComp {stat} />
        {/each}
      </div>
    {:catch}
      <p>Oops</p>
    {/await}
  </div>
  <div class="h-full w-full grid grid-cols-1 grid-rows-[55%_45%]">
    <div class="relative rounded-md overflow-y-scroll">
      <h1
        class="mb-2 text-xl font-semibold text-gray-900 dark:text-white text-left"
      >
        Current active playlist
      </h1>
      {#await activePlaylist}
        <Loader />
      {:then data}
        {#if data.data.length >= 1}
          <div class="w-full h-full flex flex-col gap-3 text-sm">
            <span class="flex gap-2 items-center justify-start">
              <p class="text-black dark:text-white">
                {data.data[0].name} :
              </p>
              <p>{(data.playedGames / data.gamesToPlay) * 100}% completed</p>
            </span>
            <span class="flex flex-wrap gap-1 text-gray-400">
              <p>{data.data[0].description},</p>
              <p>to be played for</p>
              <p class=" font-semibold text-base text-black dark:text-white">
                {data.data[0].duration}
              </p>
              <p>days.</p>
              <p>This playlist includes:</p>
            </span>
            <ul class="flex flex-col gap-2">
              {#each data.data[0].scenarios as scenarios}
                <li class="flex gap-1">
                  <p>{scenarios.scenario_name}</p>
                  <span class="flex gap-1 text-gray-400">
                    <p>for</p>
                    <p
                      class=" font-semibold text-base text-black dark:text-white"
                    >
                      {scenarios.reps}
                    </p>
                    <p>reps</p>
                  </span>
                </li>
              {/each}
            </ul>
          </div>
        {:else}
          <p
            class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-2xl text-gray-600 -rotate-45"
          >
            No active playlist
          </p>
        {/if}
      {/await}
    </div>
    <div class="w-full rounded-md flex flex-col">
      <caption
        class="rounded-t-lg p-2 text-xl font-semibold text-gray-900 dark:text-white text-left"
      >
        <p>Last 5 played games</p>
      </caption>
      <Table class="p-0">
        <TableHead class="rounded-lg">
          <TableHeadCell>NAME</TableHeadCell>
          <TableHeadCell>SCORE</TableHeadCell>
          <TableHeadCell>ACCURACY</TableHeadCell>
        </TableHead>
        <TableBody>
          {#each $dashboardHistory as game}
            <TableBodyRow>
              <TableBodyCell>{game.key_value[3].value}</TableBodyCell>
              <TableBodyCell>{game.key_value[2].value}</TableBodyCell>
              <TableBodyCell
                >{Math.floor(
                  (game.stats.hits / game.stats.shots) * 100
                )}%</TableBodyCell
              >
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  </div>
</div>
