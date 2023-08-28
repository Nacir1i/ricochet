<script lang="ts">
  let count = 1;

  //This disables contextmenu, (should prob find a better solution)
  document.addEventListener("contextmenu", (event) => event.preventDefault());
</script>

<div class="w-full h-full p-5 overflow-y-scroll no-scrollbar">
  <h1>Dashboard</h1>
  <p>{count}</p>
  <button on:click={() => count++}>click</button>
</div>
