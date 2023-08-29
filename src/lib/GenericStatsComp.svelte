<script lang="ts">
  import type { Stat } from "$lib/util";
  import Chart from "svelte-frappe-charts";

  export let stat: Stat;

  let data = {
    labels: stat.chart.dateArray,
    datasets: [
      {
        name: "Avg accuracy",
        chartType: "line",
        values: stat.chart.accuracyArray,
      },
    ],
  };
</script>

<div class=" w-[50rem] p-4 my-3 bg-gray-100 flex flex-col justify-center gap-2">
  <h1 class=" text-center text-lg">
    {stat.generic.name} ({stat.generic.games_count})
  </h1>
  <ul class=" text-sm text-black">
    <li>Average Shots :{stat.generic.shots?.toFixed(2)}</li>
    <li>Average Hits :{stat.generic.hits?.toFixed(2)}</li>
    <li>Average Accuracy :{stat.generic.accuracy?.toFixed(2)}</li>
    <li>Average damage done :{stat.generic.damage_done?.toFixed(2)}</li>
    <li>Average damage possible :{stat.generic.damage_possible?.toFixed(2)}</li>
  </ul>
  <Chart
    {data}
    type="line"
    color={["white, green"]}
    title={stat.generic.name}
  />
</div>
