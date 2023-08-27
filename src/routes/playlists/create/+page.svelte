<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { Button, Input, Select, Textarea } from "flowbite-svelte";
  import { scenarios } from "$lib";
  import { Minus, Plus } from "lucide-svelte";

  const textAreaProps = {
    id: "description",
    name: "description",
    label: "description",
    rows: 4,
    placeholder: "Description about the playlist",
  };
  const selectScenario = $scenarios.map((scenario) => ({
    value: scenario.id,
    name: scenario.name,
  }));

  let scenarioInputArray: { value: number; reps: number }[] = [];
  let name: string;
  let duration: number;
  let description: string;
</script>

<div
  class=" dark:bg-gray-600 w-full h-full p-5 overflow-scroll flex flex-col justify-start items-center gap-10"
>
  <h1 class="text-center text-xl">Create a Playlists :</h1>
  <div class="mb-6 flex flex-col gap-5 w-max-[500px] w-[500px]">
    <Input id="name" size="lg" placeholder="Playlist name" bind:value={name} />
    <Input
      id="duration"
      size="lg"
      placeholder="Duration"
      bind:value={duration}
    />
    <Textarea {...textAreaProps} bind:value={description} />
    <div class="w-full flex flex-col gap-2">
      <div class="w-full flex justify-between">
        <h1 class="text-lg">Scenarios</h1>
        <Button
          class="text-sm py-0 px-1"
          on:click={() => {
            const randomScenario = $scenarios[0];

            scenarioInputArray = [
              ...scenarioInputArray,
              { value: randomScenario.id, reps: 10 },
            ];
          }}
        >
          <Plus class="text-white w-5" />
        </Button>
      </div>
      {#each scenarioInputArray as input, index}
        <div class="w-full mb-6 flex gap-2 justify-center">
          <Select
            class="w-[70%] h-10"
            items={selectScenario}
            bind:value={input.value}
          />
          <Input
            class="w-[20%] h-10"
            id="reps"
            size="lg"
            placeholder="Reps"
            bind:value={input.reps}
          />
          <Button
            class="text-sm py-0 px-1 w-[10%]"
            on:click={() =>
              (scenarioInputArray = scenarioInputArray.filter(
                (_, i) => i !== index
              ))}
          >
            <Minus class="text-white w-5" />
          </Button>
        </div>
      {/each}
    </div>
    <Button
      on:click={async () => {
        const data = {
          name,
          description,
          duration: Number(duration),
          scenarios: scenarioInputArray.map((scenarioInput) => ({
            scenario_id: scenarioInput.value,
            reps: Number(scenarioInput.reps),
          })),
        };

        invoke("insert_playlist", { data });
      }}>Save</Button
    >
  </div>
</div>
