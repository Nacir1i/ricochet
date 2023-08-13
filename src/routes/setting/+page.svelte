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

<div>
  <h1>Settings</h1>
  <button on:click={changeDirPath}>click</button>
</div>
