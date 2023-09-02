<script lang="ts">
  import type { Stat } from "$lib/util";
  // import Chart from "svelte-frappe-charts";
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  export let stat: Stat;
  export let details: boolean = false;

  let chartCanvas: HTMLCanvasElement;

  onMount(async () => {
    const container = chartCanvas.getContext("2d");
    if (container !== null) {
      new Chart(container, {
        type: "line",
        data: {
          labels: stat.chart.dateArray,
          datasets: [
            {
              label: "avg accuracy",
              data: stat.chart.accuracyArray,
              borderColor: "#ff6f3c",
            },
          ],
        },
      });
    }
  });
</script>

<div
  class="w-full p-4 bg-gray-300 flex flex-col justify-center gap-5 text-black rounded-lg"
>
  <h1 class="text-center text-2xl font-bold">
    {stat.generic.name} ({stat.generic.games_count})
  </h1>
  {#if details}
    <ul class="text-sm grid grid-cols-3 gap-4">
      <li class="flex gap-2">
        <p class="font-semibold">Average score :</p>
        <p class=" text-gray-700">{stat.generic.score?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Shots:</p>
        <p class=" text-gray-700">{stat.generic.shots?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Hits:</p>
        <p class=" text-gray-700">{stat.generic.hits?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Accuracy:</p>
        <p class=" text-gray-700">{stat.generic.accuracy?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average damage done:</p>
        <p class=" text-gray-700">{stat.generic.damage_done?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average damage possible:</p>
        <p class=" text-gray-700">{stat.generic.damage_possible?.toFixed(2)}</p>
      </li>
    </ul>
  {/if}
  <canvas bind:this={chartCanvas} />
</div>
