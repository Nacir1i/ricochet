<script lang="ts">
  import { scenarios, set_scenarios } from "$lib";
</script>

<div class="w-full h-full p-5">
  <h1>Scenarios</h1>
  <ul class="list-disc p-5">
    {#each $scenarios as scenario}
      <li class="p-5">
        <p>id: {scenario.id}</p>
        <p>name: {scenario.name}</p>
        <p>difficulty: {scenario.difficulty}</p>
        <p>created_at: {scenario.created_at}</p>
        <p>games_count: {scenario.games_count}</p>
      </li>
    {/each}
  </ul>
</div>
