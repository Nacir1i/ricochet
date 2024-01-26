<script lang="ts">
  import logo from "$lib/asset/app-icon.png";
  import { page } from "$app/stores";
  import { Bell, ChevronDown, Settings } from "lucide-svelte";
  import { Dropdown, DropdownItem } from "flowbite-svelte";

  $: activeUrl = $page.url.pathname;
</script>

<nav
  class="w-full p-8 py-7 bg-white border-gray-200 dark:bg-gray-900 flex justify-between xl:justify-around items-center"
>
  <a href="/" class="flex items-center">
    <img src={logo} class="h-10 mr-1" alt="Ricochet Logo" />
    <span
      class="self-center text-3xl font-semibold whitespace-nowrap dark:text-white"
      >Ricochet</span
    >
  </a>
  <ul class="flex justify-center items-center gap-5 text-base">
    <li>
      <a
        href="/"
        class={activeUrl === "/" ? " border-orange-500 border-b-4" : ""}
        >Dashboard</a
      >
    </li>
    <li class="flex gap-2 cursor-pointer" id="playlist">
      <p
        class={activeUrl === "/playlists" ? "border-orange-500 border-b-4" : ""}
      >
        Playlists
      </p>
      <ChevronDown class="w-4" />
    </li>
    <li>
      <a
        href="/scenarios"
        class={activeUrl === "/scenarios"
          ? " border-orange-500 border-b-4"
          : ""}>Scenarios</a
      >
    </li>
    <li>
      <a
        href="/history"
        class={activeUrl === "/history" ? " border-orange-500 border-b-4" : ""}
        >History</a
      >
    </li>
    <Dropdown triggeredBy="#playlist" class="w-44 z-20">
      <DropdownItem href="/playlists/list">List</DropdownItem>
      <DropdownItem href="/playlists/create">Create</DropdownItem>
    </Dropdown>
  </ul>
  <div class="flex justify-center items-center gap-6">
    <button class="flex gap-2">
      <Bell class="w-5" />
      <p class="hidden xl:inline">Notifications</p>
    </button>
    <a href="/setting" class="flex gap-2">
      <Settings class="w-5" />
      <p class="hidden xl:inline">Settings</p>
    </a>
  </div>
</nav>
