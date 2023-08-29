<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { DarkMode } from "flowbite-svelte";
</script>

<div
  data-tauri-drag-region
  class="h-7 dark:bg-gray-700 bg-gray-300 select-none flex justify-end items-center fixed top-0 left-0 right-0 z-50"
>
  <div />
  <h1
    class="dark:text-white text-sm justify-self-center absolute left-1/2 -translate-x-1/2 pointer-events-none"
  >
    Ricochet
  </h1>
  <DarkMode btnClass="p-1 dark:text-white" />
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    on:click={() => appWindow.minimize()}
    class="h-7 w-7 inline-flex justify-center items-center hover:bg-slate-400"
    id=""
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="dark:text-white text-black"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      stroke-width="2"
      stroke="currentColor"
      fill="none"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path stroke="none" d="M0 0h24v24H0z" fill="none" />
      <path d="M5 12l14 0" />
    </svg>
  </div>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    on:click={() => appWindow.toggleMaximize()}
    class="h-7 w-7 inline-flex justify-center items-center hover:bg-slate-400 dark:text-white text-gray-800"
    id=""
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="dark:text-white text-black"
      width="14"
      height="14"
      viewBox="0 0 24 24"
      stroke-width="2"
      stroke="currentColor"
      fill="none"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path stroke="none" d="M0 0h24v24H0z" fill="none" />
      <path
        d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z"
      />
    </svg>
  </div>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    on:click={() => appWindow.close()}
    class="h-7 w-7 inline-flex justify-center items-center hover:bg-slate-400 dark:text-white text-gray-800"
    id=""
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="icon icon-tabler icon-tabler-x"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      stroke-width="2"
      stroke="currentColor"
      fill="none"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path stroke="none" d="M0 0h24v24H0z" fill="none" />
      <path d="M18 6l-12 12" />
      <path d="M6 6l12 12" />
    </svg>
  </div>
</div>
