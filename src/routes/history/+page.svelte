<script lang="ts">
  import { history } from "$lib";
  import Game from "$lib/game.svelte";
</script>

<div
  class="w-full h-full p-3 flex flex-wrap gap-3 content-start overflow-scroll"
>
  {#each $history as game}
    <Game {game} />
  {/each}
</div>
