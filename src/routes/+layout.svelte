<script lang="ts">
  import "../app.postcss";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { Payload, Game, MessagePayload } from "$lib/util";
  import { set_history, updateDashboardHistory, update_history } from "$lib";
  import Titlebar from "$lib/Titlebar.svelte";
  import { notifications } from "$lib/notification";
  import Toast from "$lib/Toast.svelte";
  import SideBar from "$lib/SideBar.svelte";

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
        limit: 50,
      });

      if (data) {
        set_history(data);
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
</script>

<body
  class="bg-secondary w-screen h-screen grid grid-cols-40 grid-rows-40 cursor-default overflow-hidden"
>
  <Titlebar />
  <SideBar />
  <div class="col-start-6 col-end-40 row-start-5 row-end-40 z-40">
    <slot />
  </div>
  <Toast />
</body>
