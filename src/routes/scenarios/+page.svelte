<script lang="ts">
  import { scenarios } from "$lib";
  import Scenario from "$lib/Scenario.svelte";
</script>

<div
  class="dark:bg-gray-600 w-full h-full p-5 overflow-y-scroll no-scrollbar flex flex-col justify-start items-center gap-3"
>
  <h1>Scenarios</h1>
  <div class="w-full flex flex-wrap gap-3">
    {#each $scenarios as scenario}
      <Scenario {scenario} />
    {/each}
  </div>
</div>
