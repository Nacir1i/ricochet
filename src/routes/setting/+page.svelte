<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { homeDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api";

  async function changeDirPath() {
    const selectedPath = await open({
      directory: true,
      defaultPath: await homeDir(),
    });

    console.log("selectedPath", selectedPath);
    if (!selectedPath) return;
    await invoke("update_dir_path", { path: selectedPath });
  }
</script>

<div class="w-full h-full p-5">
  <h1>Settings</h1>
  <button
    class="p-3 bg-gray-400 dark:bg-gray-600 rounded-md text-black dark:text-white"
    on:click={changeDirPath}>Update patch</button
  >
  <button
    class="p-3 bg-red-700 rounded-md text-white"
    on:click={async () => await invoke("clear_database")}>Clear database</button
  >
</div>
