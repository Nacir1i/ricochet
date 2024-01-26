<script lang="ts">
  import { Timeline, TimelineItem } from "flowbite-svelte";
  import { ChevronDown, ChevronUp } from "lucide-svelte";
  import type { PlaylistScenario } from "./util";

  export let scenario: PlaylistScenario;

  let toggle = false;
</script>

<div class="ml-2">
  <span class="flex items-center gap-2">
    <p>{scenario.scenario_name}</p>
    <button
      on:click={() => {
        toggle = !toggle;
      }}
    >
      {#if toggle}
        <ChevronUp class="cursor-pointer" />
      {:else}
        <ChevronDown class="cursor-pointer" />
      {/if}
    </button>
  </span>
  {#if toggle}
    <Timeline>
      <ul class="">
        {#each scenario.days as day, index}
          <TimelineItem date={`Day ${index + 1}`}>
            <p class="mb-4 text-base">
              You have played {day.games_count} out of {scenario.reps}
            </p>
          </TimelineItem>
        {/each}
      </ul>
    </Timeline>
  {/if}
</div>
