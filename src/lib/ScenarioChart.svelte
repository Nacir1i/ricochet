<script lang="ts">
  import {
    SCENARIO_SORTING,
    type ScenarioData,
    type ScenarioSorting,
    type ChartDataSets,
  } from "$lib/util";
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  export let scenario: ScenarioData;
  export let title = true;
  export let accuracy = false;
  export let score = true;
  export let sorting: ScenarioSorting = SCENARIO_SORTING.day;

  let chartCanvas: HTMLCanvasElement;

  function getShortDate(date: string): string {
    const input = new Date(date);
    const months = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ];

    return `${months[input.getMonth()]} ${input.getDate()}`;
  }

  function constructData() {
    let datasets = [];
    let labels: string[];
    let selectedSorting: ChartDataSets;

    switch (sorting) {
      case SCENARIO_SORTING.day:
        selectedSorting = scenario.day_data;
        labels = scenario.day_data.accuracy.map((roughData) =>
          getShortDate(roughData.date)
        );
        break;
      case SCENARIO_SORTING.month:
        selectedSorting = scenario.month_data;
        labels = scenario.month_data.accuracy.map(
          (roughData) => roughData.date
        );
        break;
      case SCENARIO_SORTING.year:
        selectedSorting = scenario.year_data;
        labels = scenario.year_data.accuracy.map((data) => data.date);
        break;
    }

    if (score) {
      datasets.push({
        label: "score",
        data: selectedSorting.score.map((data) => data.avg_score),
        borderColor: "#7dd87d",
        backgroundColor: "#7dd87d",
      });
    }
    if (accuracy) {
      datasets.push({
        label: "accuracy",
        data: selectedSorting.accuracy.map(
          (roughData) => roughData.avg_accuracy
        ),
        borderColor: "#ff6f3c",
        backgroundColor: "#ff6f3c",
      });
    }

    return { labels, datasets };
  }

  onMount(async () => {
    const container = chartCanvas.getContext("2d");

    const data = constructData();

    if (container !== null) {
      new Chart(container, {
        type: "line",
        data,
      });
    }
  });
</script>

<div
  class="h-full bg-[#CCD4F6] flex flex-col justify-around items-center text-black"
>
  {#if title}
    <h1 class="text-center text-xl">
      {scenario.name}
    </h1>
  {/if}

  <canvas bind:this={chartCanvas} class="h-full" />
</div>
