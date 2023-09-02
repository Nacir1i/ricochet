<script lang="ts">
  import type {
    ChartStats,
    FormattedChart,
    GenericStats,
    Stat,
  } from "$lib/util";
  import GenericStatsComp from "$lib/GenericStatsComp.svelte";
  import { invoke } from "@tauri-apps/api";
  import Loader from "$lib/Loader.svelte";

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

    return stats;
  }

  const stats = fetch_stats();
</script>

<div
  class="w-full h-full p-8 pt-0 overflow-y-scroll no-scrollbar flex justify-center items-start"
>
  {#await stats}
    <Loader />
  {:then stat}
    <div class="w-[80%] flex gap-4 flex-col">
      {#each stat as data}
        <GenericStatsComp details stat={data} />
      {/each}
    </div>
  {:catch error}
    <p class="text-red-500">{error.message}</p>
  {/await}
</div>
