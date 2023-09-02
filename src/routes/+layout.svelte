<script lang="ts">
  import "../app.postcss";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { Payload, Game, Scenario, MessagePayload } from "$lib/util";
  import {
    setDashboardHistory,
    set_history,
    set_scenarios,
    updateDashboardHistory,
    update_history,
  } from "$lib";
  import Titlebar from "$lib/Titlebar.svelte";
  import { notifications } from "$lib/notification";
  import Toast from "$lib/Toast.svelte";
  import Navbar from "$lib/Navbar.svelte";

  async function newRunListener() {
    await listen("new_run", (event) => {
      const payload = event.payload as Payload;

      update_history(payload.data);
      updateDashboardHistory(payload.data);

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
        limit: 20,
      });

      if (data) {
        set_history(data);
        setDashboardHistory(data.slice(0, 5));
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
</script>

<body
  class="pt-7 dark:bg-gray-900 light:bg-gray-200 w-screen h-screen flex flex-col cursor-default dark:text-white"
>
  <Titlebar />
  <Navbar />
  <slot />
  <Toast />
</body>
