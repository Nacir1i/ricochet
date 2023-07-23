<script lang="ts">
  import { history } from "$lib";
</script>

<div>
  {#each $history as game}
    <h1>{game.stats.weapon}</h1>
    <ul>
      {#each game.key_value as { key, value }}
        <li>{key} {value}</li>
      {/each}
    </ul>
  {/each}
</div>
