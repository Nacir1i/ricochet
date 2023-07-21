<script lang="ts">
  let count = 1;
</script>

<div>
  <h1>Settings</h1>
  <p>{count}</p>
  <button on:click={() => count++}>click</button>
</div>
