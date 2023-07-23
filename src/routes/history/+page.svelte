<script lang="ts">
  import { history } from "$lib";
  import Game from "$lib/game.svelte";
</script>

<div class="w-full h-full p-3 flex flex-wrap gap-3 content-start">
  {#each [1, 2, 3, 4] as game}
    <Game />
  {/each}
</div>
