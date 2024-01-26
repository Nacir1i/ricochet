<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { Minus, X } from "lucide-svelte";
  import title from "$lib/asset/title.png";
</script>

<div
  data-tauri-drag-region
  class="select-none col-start-2 col-end-40 row-span-4 flex justify-between items-center z-50"
>
  <img src={title} alt="Ricochet Logo" class="pointer-events-none" />
  <div class="flex items-center gap-5">
    <button
      style="clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);"
      on:click={() => appWindow.minimize()}
      class="p-[0.4rem] inline-flex justify-center items-center bg-third relative"
    >
      <Minus class="text-accent w-5" />
    </button>
    <button
      style="clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);"
      on:click={() => appWindow.close()}
      class="p-[0.4rem] inline-flex justify-center items-center bg-third"
    >
      <X class="text-accent w-5" />
    </button>
  </div>
</div>
