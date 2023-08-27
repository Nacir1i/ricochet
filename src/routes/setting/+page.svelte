<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { homeDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api";
  import type { Settings } from "$lib/util";

  async function changeDirPath() {
    const selectedPath = await open({
      directory: true,
      defaultPath: await homeDir(),
    });

    if (!selectedPath) return;
    await invoke("update_dir_path", { path: selectedPath });
  }

  async function fetch_settings(): Promise<Settings> {
    return await invoke("fetch_settings");
  }

  let settings = fetch_settings();
</script>

<div
  class="dark:bg-gray-600 w-full h-full p-5 flex flex-col gap-6 overflow-scroll"
>
  <h1>Settings</h1>
  {#await settings}
    <p>...loading</p>
  {:then setting}
    <div class="w-full h-10 flex gap-5">
      <input
        type="text"
        disabled
        readonly
        value={setting.directory_path}
        class=" dark:bg-white bg-gray-200 border-none rounded-md text-black dark:text-gray-400 w-[35rem]"
      />
      <button
        class="px-3 bg-[#ff6f3c] rounded-md text-white text-sm"
        on:click={changeDirPath}>Update path</button
      >
    </div>
    <button
      class="p-3 bg-red-700 rounded-md text-white"
      on:click={async () => await invoke("clear_database")}
      >Clear database</button
    >
  {/await}
</div>
