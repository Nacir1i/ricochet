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

  let spanClass = "flex-1 ml-3 whitespace-nowrap";
  let site = {
    name: "Ricochet",
    href: "/",
    img: logo,
  };

  $: activeUrl = $page.url.pathname;
</script>

<body class="dark:bg-gray-600 light:bg-gray-200 w-screen h-screen">
  <Sidebar asideClass="w-64 h-full">
    <SidebarWrapper
      class="flex flex-col justify-between"
      divClass="h-full overflow-y-auto py-4 px-3 bg-gray-50 dark:bg-gray-800"
    >
      <SidebarGroup>
        <SidebarBrand class="my-5" {site} />
        <SidebarItem
          label="Dashboard"
          active={activeUrl === ("/" || "/dashboard")}
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
        <SidebarItem label="History" {spanClass}>
          <svelte:fragment slot="icon">
            <svg
              class="w-6 h-6 text-gray-800 dark:text-white"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                d="m7.164 3.805-4.475.38L.327 6.546a1.114 1.114 0 0 0 .63 1.89l3.2.375 3.007-5.006ZM11.092 15.9l.472 3.14a1.114 1.114 0 0 0 1.89.63l2.36-2.362.38-4.475-5.102 3.067Zm8.617-14.283A1.613 1.613 0 0 0 18.383.291c-1.913-.33-5.811-.736-7.556 1.01-1.98 1.98-6.172 9.491-7.477 11.869a1.1 1.1 0 0 0 .193 1.316l.986.985.985.986a1.1 1.1 0 0 0 1.316.193c2.378-1.3 9.889-5.5 11.869-7.477 1.746-1.745 1.34-5.643 1.01-7.556Zm-3.873 6.268a2.63 2.63 0 1 1-3.72-3.72 2.63 2.63 0 0 1 3.72 3.72Z"
              />
            </svg>
          </svelte:fragment>
          <svelte:fragment slot="subtext">
            <span
              class="inline-flex justify-center items-center p-3 ml-3 w-3 h-3 text-sm font-medium text-primary-600 bg-primary-200 rounded-full dark:bg-primary-900 dark:text-primary-200"
              >3</span
            >
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem label="Stats" {spanClass}>
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
        <SidebarItem label="Profile">
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
        <SidebarItem label="Settings">
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
