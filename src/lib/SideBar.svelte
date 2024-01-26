<script lang="ts">
  import { page } from "$app/stores";
  import {
    Bell,
    Settings,
    History,
    Home,
    AlignJustify,
    Crosshair,
  } from "lucide-svelte";
  import { Tooltip } from "flowbite-svelte";

  $: activeUrl = $page.url.pathname;
</script>

<nav
  class="col-start-2 col-end-5 row-start-5 row-end-40 flex flex-col justify-center items-center z-50"
>
  <div class="w-full h-full flex flex-col justify-between items-center">
    <div class="w-full h-[9%] rounded-t-full bg-third" />
    <ul
      class="w-full h-[80%] bg-third flex flex-col justify-start items-center gap-5 text-base text-gray-50/50"
    >
      <li>
        <a href="/">
          <Home class={activeUrl === "/" ? "w-8 h-8 text-accent" : "h-8 w-8"} />
        </a>
        <Tooltip>Home</Tooltip>
      </li>
      <li class="flex gap-2 cursor-pointer" id="playlist">
        <a href="/playlists/list">
          <AlignJustify
            class={activeUrl === "/playlists/list"
              ? "w-8 h-8 text-accent"
              : "w-8 h-8"}
          />
        </a>
        <Tooltip>Playlists</Tooltip>
      </li>
      <li>
        <a href="/scenarios">
          <Crosshair
            class={activeUrl === "/scenarios"
              ? "w-8 h-8 text-accent"
              : "w-8 h-8"}
          />
        </a>
        <Tooltip>Scenarios</Tooltip>
      </li>
      <li>
        <a href="/history">
          <History
            class={activeUrl === "/history" ? "w-8 h-8 text-accent" : "w-8 h-8"}
          />
        </a>
        <Tooltip>History</Tooltip>
      </li>
    </ul>
    <div
      class="w-full bg-third flex flex-col justify-center items-center gap-6 text-gray-50/50"
    >
      <!-- <button class="flex gap-2">
        <Bell />
      </button> -->
      <a href="/setting" class="flex gap-2">
        <Settings class="w-8 h-8" />
      </a>
    </div>
    <div class="w-full h-[9%] rounded-b-full bg-third" />
  </div>
</nav>
