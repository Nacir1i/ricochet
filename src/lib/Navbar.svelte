<script lang="ts">
  import logo from "$lib/asset/app-icon.png";
  import { page } from "$app/stores";
  import { Bell, ChevronDown, Search, Settings } from "lucide-svelte";
  import { Dropdown, DropdownItem } from "flowbite-svelte";

  const menu = [
    {
      label: "Dashboard",
      href: "/",
    },
    {
      label: "Playlists",
      href: "/playlists/list",
    },
    {
      label: "Scenarios",
      href: "/scenarios",
    },
    {
      label: "History",
      href: "/history",
    },
    {
      label: "Stats",
      href: "/stats",
    },
  ];

  $: activeUrl = $page.url.pathname;
</script>

<nav
  class="w-full p-4 py-7 bg-white border-gray-200 dark:bg-gray-900 flex justify-between xl:justify-around items-center"
>
  <div class="flex justify-center items-center gap-5">
    <a href="/" class="flex items-center">
      <img src={logo} class="h-6 mr-1" alt="Ricochet Logo" />
      <span
        class="self-center text-xl font-semibold whitespace-nowrap dark:text-white"
        >Ricochet</span
      >
    </a>
    <div
      class="flex justify-center items-center gap-2 dark:bg-gray-700 bg-gray-300 px-3 rounded-lg"
    >
      <input
        type="text"
        name=""
        id=""
        class="dark:bg-gray-700 bg-gray-300 rounded-lg text-white border-none w-40 h-10"
        placeholder="Search..."
      />
      <button>
        <Search class="text-white text-sm w-5" />
      </button>
    </div>
  </div>
  <div
    class=" 2xl:w-[55%] flex justify-between items-center gap-16 dark:text-white"
  >
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
          class={activeUrl === "/playlists"
            ? "border-orange-500 border-b-4"
            : ""}
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
          class={activeUrl === "/history"
            ? " border-orange-500 border-b-4"
            : ""}>History</a
        >
      </li>
      <li>
        <a
          href="/stats"
          class={activeUrl === "/stats" ? " border-orange-500 border-b-4" : ""}
          >Stats</a
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
  </div>
</nav>
