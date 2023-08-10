<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { homeDir } from "@tauri-apps/api/path";

  async function changeDirPath() {
    const selectedPath = await open({
      directory: true,
      defaultPath: await homeDir(),
    });

    console.log("selectedPath", selectedPath);
  }
</script>

<div>
  <h1>Settings</h1>
  <button on:click={changeDirPath}>click</button>
</div>
