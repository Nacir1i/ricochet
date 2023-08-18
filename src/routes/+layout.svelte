<script lang="ts">
  import "../app.postcss";
  import {
    Sidebar,
    SidebarGroup,
    SidebarItem,
    SidebarWrapper,
    SidebarCta,
    SidebarBrand,
    DarkMode,
  } from "flowbite-svelte";
  import logo from "$lib/asset/dev_logo.jpg";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { Payload, Game } from "$lib/util";
  import { set_history, update_history } from "$lib";
  import Titlebar from "$lib/titlebar.svelte";

  let spanClass = "flex-1 ml-3 whitespace-nowrap";
  let site = {
    name: "Ricochet",
    href: "/",
    img: logo,
  };

  async function eventListener() {
    await listen("new_run", (event) => {
      const payload = event.payload as Payload;

      update_history(payload.data);
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
      console.log("data", data);
    } catch (error) {
      console.error(error);
    }
  }

  eventListener();
  fetch_data();
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
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z"
              /><path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z"
              /></svg
            >
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Scenarios"
          active={activeUrl === "/scenarios"}
          href="/scenarios"
          {spanClass}
        >
          <svelte:fragment slot="icon">
            <svg
              class="w-6 h-6 stoke-gray-800 dark:stoke-white"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              ><g id="SVGRepo_bgCarrier" stroke-width="0" /><g
                id="SVGRepo_tracerCarrier"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke="#ffff"
              /><g id="SVGRepo_iconCarrier">
                <path
                  d="M19 1C19 0.447715 19.4477 0 20 0C20.5523 0 21 0.447715 21 1V1.58582L22.2709 0.314894C22.6614 -0.0756305 23.2946 -0.0756294 23.6851 0.314895C24.0757 0.705419 24.0757 1.33858 23.6851 1.72911L22.4142 3H23C23.5523 3 24 3.44772 24 4C24 4.55228 23.5523 5 23 5H20.4142L12.7017 12.7125C12.3112 13.103 11.678 13.103 11.2875 12.7125C10.897 12.322 10.897 11.6888 11.2875 11.2983L19 3.58582V1Z"
                  fill="#ffff"
                />
                <path
                  d="M17.3924 3.78908C17.834 3.3475 17.7677 2.61075 17.2182 2.31408C15.6655 1.47581 13.8883 1 12 1C5.92487 1 1 5.92487 1 12C1 18.0751 5.92487 23 12 23C18.0751 23 23 18.0751 23 12C23 10.1154 22.5261 8.34153 21.6909 6.79102C21.3946 6.24091 20.6574 6.17424 20.2155 6.61606L20.1856 6.64598C19.8554 6.97615 19.8032 7.48834 20.016 7.90397C20.6451 9.1326 21 10.5249 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C13.4782 3 14.8732 3.35638 16.1037 3.98791C16.5195 4.20129 17.0322 4.14929 17.3627 3.81884L17.3924 3.78908Z"
                  fill="#ffff"
                />
                <path
                  d="M14.3899 6.79159C14.8625 6.31902 14.7436 5.52327 14.1062 5.32241C13.4415 5.11295 12.7339 5 12 5C8.13401 5 5 8.13401 5 12C5 15.866 8.13401 19 12 19C15.866 19 19 15.866 19 12C19 11.2702 18.8883 10.5664 18.6811 9.9049C18.4811 9.26659 17.6846 9.14697 17.2117 9.61995L17.1194 9.71224C16.8382 9.99337 16.7595 10.4124 16.8547 10.7984C16.9496 11.1833 17 11.5858 17 12C17 14.7614 14.7614 17 12 17C9.23858 17 7 14.7614 7 12C7 9.23858 9.23858 7 12 7C12.4172 7 12.8225 7.0511 13.21 7.1474C13.5965 7.24347 14.0166 7.16496 14.2982 6.88331L14.3899 6.79159Z"
                  fill="#ffff"
                />
                <path
                  d="M11.078 9.15136C11.4874 9.01484 11.6934 9.48809 11.3882 9.79329L10.5827 10.5989C9.80254 11.379 9.80254 12.6438 10.5827 13.4239C11.3628 14.204 12.6276 14.204 13.4077 13.4239L14.2031 12.6285C14.5089 12.3227 14.9822 12.5301 14.8429 12.9397C14.441 14.1209 13.3135 15 12 15C10.3431 15 9 13.6569 9 12C9 10.6796 9.88827 9.54802 11.078 9.15136Z"
                  fill="#ffff"
                />
              </g></svg
            >
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="History"
          active={activeUrl === "/history"}
          href="/history"
          {spanClass}
        >
          <svelte:fragment slot="icon">
            <svg
              class="w-6 h-6 stoke-gray-800 dark:stoke-white"
              viewBox="0 0 24.00 24.00"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              ><g id="SVGRepo_bgCarrier" stroke-width="0" /><g
                id="SVGRepo_tracerCarrier"
                stroke-linecap="round"
                stroke-linejoin="round"
              /><g id="SVGRepo_iconCarrier">
                <path
                  d="M12 8V12L14.5 14.5"
                  stroke="#ffffff"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
                <path
                  d="M5.60423 5.60423L5.0739 5.0739V5.0739L5.60423 5.60423ZM4.33785 6.87061L3.58786 6.87438C3.58992 7.28564 3.92281 7.61853 4.33408 7.6206L4.33785 6.87061ZM6.87963 7.63339C7.29384 7.63547 7.63131 7.30138 7.63339 6.88717C7.63547 6.47296 7.30138 6.13549 6.88717 6.13341L6.87963 7.63339ZM5.07505 4.32129C5.07296 3.90708 4.7355 3.57298 4.32129 3.57506C3.90708 3.57715 3.57298 3.91462 3.57507 4.32882L5.07505 4.32129ZM3.75 12C3.75 11.5858 3.41421 11.25 3 11.25C2.58579 11.25 2.25 11.5858 2.25 12H3.75ZM16.8755 20.4452C17.2341 20.2378 17.3566 19.779 17.1492 19.4204C16.9418 19.0619 16.483 18.9393 16.1245 19.1468L16.8755 20.4452ZM19.1468 16.1245C18.9393 16.483 19.0619 16.9418 19.4204 17.1492C19.779 17.3566 20.2378 17.2341 20.4452 16.8755L19.1468 16.1245ZM5.14033 5.07126C4.84598 5.36269 4.84361 5.83756 5.13505 6.13191C5.42648 6.42626 5.90134 6.42862 6.19569 6.13719L5.14033 5.07126ZM18.8623 5.13786C15.0421 1.31766 8.86882 1.27898 5.0739 5.0739L6.13456 6.13456C9.33366 2.93545 14.5572 2.95404 17.8017 6.19852L18.8623 5.13786ZM5.0739 5.0739L3.80752 6.34028L4.86818 7.40094L6.13456 6.13456L5.0739 5.0739ZM4.33408 7.6206L6.87963 7.63339L6.88717 6.13341L4.34162 6.12062L4.33408 7.6206ZM5.08784 6.86684L5.07505 4.32129L3.57507 4.32882L3.58786 6.87438L5.08784 6.86684ZM12 3.75C16.5563 3.75 20.25 7.44365 20.25 12H21.75C21.75 6.61522 17.3848 2.25 12 2.25V3.75ZM12 20.25C7.44365 20.25 3.75 16.5563 3.75 12H2.25C2.25 17.3848 6.61522 21.75 12 21.75V20.25ZM16.1245 19.1468C14.9118 19.8483 13.5039 20.25 12 20.25V21.75C13.7747 21.75 15.4407 21.2752 16.8755 20.4452L16.1245 19.1468ZM20.25 12C20.25 13.5039 19.8483 14.9118 19.1468 16.1245L20.4452 16.8755C21.2752 15.4407 21.75 13.7747 21.75 12H20.25ZM6.19569 6.13719C7.68707 4.66059 9.73646 3.75 12 3.75V2.25C9.32542 2.25 6.90113 3.32791 5.14033 5.07126L6.19569 6.13719Z"
                  fill="#ffffff"
                />
              </g></svg
            >
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Stats"
          active={activeUrl === "/stats"}
          href="/stats"
        >
          <svelte:fragment slot="icon">
            <svg
              class="w-6 h-6 text-gray-800 dark:text-white"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 17 18"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M1 12v5m5-9v9m5-5v5m5-9v9M1 7l5-6 5 6 5-6"
              />
            </svg>
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem
          label="Profile"
          active={activeUrl === "/profile"}
          href="profile"
        >
          <svelte:fragment slot="icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
              /></svg
            >
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
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M4.5 12a7.5 7.5 0 0015 0m-15 0a7.5 7.5 0 1115 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077l1.41-.513m14.095-5.13l1.41-.513M5.106 17.785l1.15-.964m11.49-9.642l1.149-.964M7.501 19.795l.75-1.3m7.5-12.99l.75-1.3m-6.063 16.658l.26-1.477m2.605-14.772l.26-1.477m0 17.726l-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205L12 12m6.894 5.785l-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864l-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495"
              /></svg
            >
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem label="Theme">
          <svelte:fragment slot="icon">
            <DarkMode btnClass="p-1" />
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
</body>
