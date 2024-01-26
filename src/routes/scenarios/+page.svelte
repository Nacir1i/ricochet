<script lang="ts">
  import {
    SCENARIO_SORTING,
    type ScenarioData,
    type ScenarioSorting,
  } from "$lib/util";
  import ScenarioChart from "$lib/ScenarioChart.svelte";
  import { invoke } from "@tauri-apps/api";
  import Loader from "$lib/Loader.svelte";
  import PrimaryContainer from "$lib/PrimaryContainer.svelte";
  import { Sparkles } from "lucide-svelte";

  let selectedScenario: ScenarioData | null;
  let score: boolean | null = true;
  let accuracy: boolean | null = false;
  let sorting: ScenarioSorting | null;

  async function fetch_scenarios(): Promise<ScenarioData[] | []> {
    const data = await invoke<ScenarioData[]>("fetch_scenario_data");

    if (data.length > 0) {
      selectedScenario = data[0];

      return data;
    }

    return [];
  }

  const scenarios = fetch_scenarios();
</script>

<PrimaryContainer>
  <div class="absolute top-2 left-[19rem] flex gap-4">
    <label
      class="relative inline-flex items-center cursor-pointer border-none focus:ring-0"
    >
      <input
        type="checkbox"
        value=""
        class="sr-only peer"
        bind:checked={accuracy}
        on:change={() => {
          const prev = accuracy;
          accuracy = null;
          setTimeout(() => {
            accuracy = prev;
          }, 1);
        }}
      />
      <div
        class="w-8 h-4 flex bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[0px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-accent"
      />
      <span class="ml-1 text-sm font-medium text-gray-900 dark:text-gray-300"
        >Accuracy</span
      >
    </label>

    <label
      class="relative inline-flex items-center cursor-pointer focus:outline-none"
    >
      <input
        type="checkbox"
        value=""
        class="sr-only peer"
        bind:checked={score}
        on:change={() => {
          const prev = score;
          score = null;
          setTimeout(() => {
            score = prev;
          }, 1);
        }}
      />
      <div
        class="w-8 h-4 flex bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[0px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-accent"
      />
      <span class="ml-1 text-sm font-medium text-gray-900 dark:text-gray-300"
        >Score</span
      >
    </label>
  </div>
  <div
    class="[clip-path:polygon(100%_100%,100%_0,0_100%)] absolute -top-[1.34rem] right-[3.2rem] w-[2.7rem] h-[2.7rem] rotate-45 bg-accent"
  />
  <div
    class="z-50 absolute top-0 right-[4.5rem] bg-accent flex justify-between items-center w-[15rem] gap-3 p-1 py-[0.35rem] text-white text-sm"
  >
    <button
      class="flex-1 bg-primary text-center"
      on:click={() => {
        sorting = null;
        setTimeout(() => {
          sorting = SCENARIO_SORTING.day;
        }, 1);
      }}>day</button
    >
    <button
      class="flex-1 bg-primary text-center"
      on:click={() => {
        sorting = null;
        setTimeout(() => {
          sorting = SCENARIO_SORTING.month;
        }, 1);
      }}>month</button
    >
    <button
      class="flex-1 bg-primary text-center"
      on:click={() => {
        sorting = null;
        setTimeout(() => {
          sorting = SCENARIO_SORTING.year;
        }, 1);
      }}>year</button
    >
  </div>
  <div
    class="[clip-path:polygon(100%_100%,100%_0,0_100%)] absolute -top-[1.34rem] right-[18.2rem] w-[2.7rem] h-[2.7rem] rotate-45 bg-accent"
  />
  <div class="w-full h-full flex gap-[4rem]">
    <div
      class="h-full w-[27%] overflow-y-scroll scrollbar-thin scrollbar-thumb-accent bg-[#CCD4F6]"
    >
      {#await scenarios}
        <Loader />
      {:then scenarios}
        <ul class="p-2">
          {#each scenarios as scenario}
            <li class="mb-2 px-2">
              <button
                on:click={() => {
                  selectedScenario = null;
                  setTimeout(() => {
                    selectedScenario = scenario;
                  }, 1);
                }}>{scenario.name}</button
              >
            </li>
          {/each}
        </ul>
      {/await}
    </div>
    <div class="h-full w-[63%]">
      <div class="w-full h-full bg-[#CCD4F6] grid grid-cols-1 grid-rows-7 p-5">
        {#if selectedScenario && accuracy !== null && score !== null && sorting !== null}
          <div class="col-span-1 row-start-1 row-end-4 relative">
            <h1 class="text-center text-2xl font-bold">
              {selectedScenario.name}
            </h1>
            <span
              class="flex justify-center items-center gap-1 mb-5 text-xs text-gray-500"
            >
              <Sparkles class="text-green-500 w-4" />
              <p>Best score:</p>
              <p class="font-bold text-gray-700">
                {selectedScenario.max_score}
              </p>
            </span>
            <div class="px-9">
              <ul class="grid grid-cols-2 gap-3">
                <li class="text-sm flex gap-2">
                  <p>Avg Score:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario?.score)}
                  </p>
                </li>
                <li class="text-sm flex gap-2">
                  <p>Avg Accuracy:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario.accuracy)}%
                  </p>
                </li>
                <li class="text-sm flex gap-2">
                  <p>Avg Hits:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario.hits)}
                  </p>
                </li>
                <li class="text-sm flex gap-2">
                  <p>Avg Shots:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario.shots)}
                  </p>
                </li>
                <li class="text-sm flex gap-2">
                  <p>Avg DMG done:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario.damage_done)}
                  </p>
                </li>
                <li class="text-sm flex gap-2">
                  <p>Avg DMG possible:</p>
                  <p class="text-gray-500">
                    {Math.floor(selectedScenario.damage_possible)}
                  </p>
                </li>
              </ul>
            </div>
          </div>
          <div class="col-span-1 row-start-4 row-span-4">
            <ScenarioChart
              title={false}
              scenario={selectedScenario}
              {accuracy}
              {score}
              {sorting}
            />
          </div>
        {/if}
      </div>
    </div>
  </div>
</PrimaryContainer>
