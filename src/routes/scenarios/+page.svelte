<script lang="ts">
  import type { ScenarioData } from "$lib/util";
  import GenericStatsComp from "$lib/Scenario.svelte";
  import { invoke } from "@tauri-apps/api";
  import Loader from "$lib/Loader.svelte";

  async function fetch_scenarios(): Promise<ScenarioData[]> {
    const data = await invoke<ScenarioData[]>("fetch_scenario_data");
    console.log(data);

    return data;
  }

  const scenarios = fetch_scenarios();
</script>

<div
  class="w-full h-full p-8 pt-0 overflow-y-scroll no-scrollbar flex justify-center items-start"
>
  {#await scenarios}
    <Loader />
  {:then scenarios}
    <div class="w-[80%] flex gap-4 flex-col">
      {#each scenarios as scenario}
        <GenericStatsComp details {scenario} />
      {/each}
    </div>
  {:catch error}
    <p class="text-red-500">{error.message}</p>
  {/await}
</div>
