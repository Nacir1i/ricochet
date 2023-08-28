<script lang="ts">
  import type { Scenario } from "./util";

  export let scenario: Scenario;
</script>

<div
  class="w-[350px] max-w-[350px] p-5 dark:bg-gray-800 bg-gray-300 dark:text-white rounded-lg"
>
  <span class="w-full flex items-center justify-center gap-2 truncate">
    <p>{scenario.name}</p>
  </span>
  <span class="flex items-center justify-start gap-2">
    <p>Difficulty:</p>
    <p>{scenario.difficulty}</p>
  </span>
  <span class="flex items-center justify-start gap-2">
    <p>First played in:</p>
    <p>{scenario.created_at}</p>
  </span>
  <span class="flex items-center justify-start gap-2">
    <p>Games played:</p>
    <p>{scenario.games_count}</p>
  </span>
</div>
