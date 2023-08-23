<script>
  // @ts-nocheck

  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";
  import { notifications } from "./notification";

  export let themes = {
    danger: "#E26D69",
    success: "#84C991",
    warning: "#f0ad4e",
    info: "#5bc0de",
    default: "#aaaaaa",
  };
</script>

<div
  class="fixed top-10 left-0 right-0 mx-auto p-0 z-50 flex flex-col justify-start items-center pointer-events-none"
>
  {#each $notifications as notification (notification.id)}
    <div
      animate:flip
      class="flex-none mb-3 rounded-lg"
      style="background: {themes[notification.type]};"
      transition:fly={{ y: 30 }}
    >
      <div class=" p-3 block text-white font-semibold">
        {notification.message}
      </div>
      {#if notification.icon}<i class={notification.icon} />{/if}
    </div>
  {/each}
</div>
