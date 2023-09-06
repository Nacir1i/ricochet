<script lang="ts">
  import type { ScenarioData } from "$lib/util";
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  export let scenario: ScenarioData;
  export let details: boolean = false;

  let chartCanvas: HTMLCanvasElement;

  onMount(async () => {
    const container = chartCanvas.getContext("2d");
    const data = {
      labels: scenario.day_data.accuracy.map((roughData) => roughData.date),
      datasets: [
        {
          label: "accuracy",
          data: scenario.day_data.accuracy.map(
            (roughData) => roughData.avg_accuracy
          ),
        },
        {
          label: "score",
          data: scenario.day_data.score.map((roughData) => roughData.avg_score),
        },
      ],
    };

    if (container !== null) {
      new Chart(container, {
        type: "line",
        data,
      });
    }
  });
</script>

<div
  class="w-full p-4 bg-gray-300 flex flex-col justify-center gap-5 text-black rounded-lg"
>
  <h1 class="text-center text-2xl font-bold">
    {scenario.name} ({scenario.games_count})
  </h1>
  {#if details}
    <ul class="text-sm grid grid-cols-3 gap-4">
      <li class="flex gap-2">
        <p class="font-semibold">Average score :</p>
        <p class=" text-gray-700">{scenario.score?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Shots:</p>
        <p class=" text-gray-700">{scenario.shots?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Hits:</p>
        <p class=" text-gray-700">{scenario.hits?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average Accuracy:</p>
        <p class=" text-gray-700">{scenario.accuracy?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average damage done:</p>
        <p class=" text-gray-700">{scenario.damage_done?.toFixed(2)}</p>
      </li>
      <li class="flex gap-2">
        <p class="font-semibold">Average damage possible:</p>
        <p class=" text-gray-700">{scenario.damage_possible?.toFixed(2)}</p>
      </li>
    </ul>
  {/if}
  <canvas bind:this={chartCanvas} />
</div>
