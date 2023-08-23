<script lang="ts">
  let count = 1;
</script>

<div class="w-full h-full p-5 overflow-scroll">
  <h1>Profile</h1>
  <p>{count}</p>
  <button on:click={() => count++}>click</button>
</div>
