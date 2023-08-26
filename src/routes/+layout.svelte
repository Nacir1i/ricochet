<script lang="ts">
  import "../app.postcss";
  import {
    Sidebar,
    SidebarGroup,
    SidebarItem,
    SidebarWrapper,
    SidebarCta,
    SidebarBrand,
  } from "flowbite-svelte";
  import logo from "$lib/asset/app-icon.png";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { Payload, Game, Scenario, MessagePayload } from "$lib/util";
  import { set_history, set_scenarios, update_history } from "$lib";
  import Titlebar from "$lib/Titlebar.svelte";
  import {
    PieChart,
    History,
    LineChart,
    Target,
    User2,
    Settings,
    List,
  } from "lucide-svelte";
  import { notifications } from "$lib/notification";
  import Toast from "$lib/Toast.svelte";

  let spanClass = "flex-1 ml-3 whitespace-nowrap";
  let site = {
    name: "Ricochet",
    href: "/",
    img: logo,
  };

  async function newRunListener() {
    await listen("new_run", (event) => {
      const payload = event.payload as Payload;

      update_history(payload.data);

      console.log("new run", payload);
      notifications.success("Run saved", 2000);
    });
  }

  async function errorListener() {
    await listen("error", (event) => {
      const payload = event.payload as MessagePayload;

      notifications.danger(payload.data, 3000);
    });
  }

  async function infoListener() {
    await listen("info", (event) => {
      const payload = event.payload as MessagePayload;

      notifications.info(payload.data, 2000);
    });
  }

  async function warningListener() {
    await listen("warning", (event) => {
      const payload = event.payload as MessagePayload;

      notifications.warning(payload.data, 4000);
    });
  }

  async function fetch_data() {
    try {
      const data = await invoke<Game[]>("fetch_game_page", {
        page: 1,
        limit: 50,
      });

      console.log("data", data);

      if (data) {
        set_history(data);
      }
    } catch (error) {
      console.error(error);
    }
  }
  async function fetchScenarios() {
    try {
      const data: Scenario[] = await invoke("fetch_scenarios");

      if (data) {
        set_scenarios(data);
      }
    } catch (error) {
      console.error(error);
    }
  }

  newRunListener();
  errorListener();
  infoListener();
  warningListener();
  fetch_data();
  fetchScenarios();
  $: activeUrl = $page.url.pathname;
</script>

<body class="pt-7 dark:bg-gray-300 light:bg-gray-200 w-screen h-screen flex">
  <Titlebar />
  <Sidebar class=" min-w-[16rem]">
    <SidebarWrapper
      class="flex flex-col justify-between bg-gray-100"
      divClass="h-full overflow-y-auto py-4 px-3 bg-gray-50 dark:bg-gray-800"
    >
      <SidebarGroup>
        <SidebarBrand class="my-5" {site} />
        <SidebarItem
          label="Dashboard"
          active={activeUrl === ("/" || "/dashboard")}
          href="/"
        >
          <svelte:fragment slot="icon">
            <PieChart class="w-6 h-6 stoke-gray-800 dark:stoke-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Playlists"
          active={activeUrl === "/playlists"}
          href="/playlists"
        >
          <svelte:fragment slot="icon">
            <List class="w-6 h-6 stoke-gray-800 dark:stoke-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Scenarios"
          active={activeUrl === "/scenarios"}
          href="/scenarios"
          {spanClass}
        >
          <svelte:fragment slot="icon">
            <Target class="w-6 h-6 stoke-gray-800 dark:stoke-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="History"
          active={activeUrl === "/history"}
          href="/history"
          {spanClass}
        >
          <svelte:fragment slot="icon">
            <History class="w-6 h-6 text-gray-800 dark:text-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Stats"
          active={activeUrl === "/stats"}
          href="/stats"
        >
          <svelte:fragment slot="icon">
            <LineChart class="w-6 h-6 text-gray-800 dark:text-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Profile"
          active={activeUrl === "/profile"}
          href="profile"
        >
          <svelte:fragment slot="icon">
            <User2 class="w-6 h-6 stoke-gray-800 dark:stoke-white" />
          </svelte:fragment>
        </SidebarItem>
      </SidebarGroup>
      <SidebarGroup>
        <SidebarItem
          label="Settings"
          active={activeUrl === "/setting"}
          href="setting"
        >
          <svelte:fragment slot="icon">
            <Settings class="w-6 h-6 stoke-gray-800 dark:stoke-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarCta label="Beta">
          <svelte:fragment slot="icon" />
          <p class="mb-3 text-sm text-primary-900 dark:text-primary-400">
            This app is still in production, with many more features to come
          </p>
        </SidebarCta>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>
  <slot />
  <Toast />
</body>
