<script lang="ts">
  import { history } from "$lib";
  import PrimaryContainer from "$lib/PrimaryContainer.svelte";

  let searchTerm = "";
  let helper = { start: 1, end: 20, total: "X" };

  $: filteredItems = $history.filter(
    (item) =>
      item.key_value[3].value
        .toLowerCase()
        .indexOf(searchTerm.toLowerCase()) !== -1
  );
</script>

<PrimaryContainer>
  <div
    class="h-full w-full relative overflow-x-auto shadow-md overflow-scroll scrollbar-thin scrollbar-thumb-accent"
  >
    <div class="w-full flex justify-between">
      <div class="w-56 mb-2">
        <label for="search" class="mb-2 text-sm font-medium sr-only text-white"
          >Search</label
        >
        <div class="relative">
          <div
            class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none"
          >
            <svg
              class="w-4 h-4 text-gray-400"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 20 20"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
              />
            </svg>
          </div>
          <input
            type="search"
            id="search"
            class="block w-full p-2 pl-10 text-sm text-gray-400 border border-none bg-secondary focus:ring-0"
            placeholder="Search"
            required
            bind:value={searchTerm}
          />
        </div>
      </div>
    </div>
    <table class="w-full text-sm text-left text-blue-100 dark:text-blue-100">
      <thead class="text-xs text-white uppercase bg-third dark:text-white">
        <tr>
          <th scope="col" class="px-6 py-3"> NAME </th>
          <th scope="col" class="px-6 py-3"> SCORE </th>
          <th scope="col" class="px-6 py-3"> KILLS </th>
          <th scope="col" class="px-6 py-3"> ACCURACY </th>
          <th scope="col" class="px-6 py-3"> SENSE </th>
          <th scope="col" class="px-6 py-3"> FOV </th>
        </tr>
      </thead>
      <tbody>
        {#each filteredItems as game}
          <tr class="bg-secondary">
            <th
              scope="row"
              class="px-6 py-4 font-medium text-blue-50 whitespace-nowrap dark:text-blue-100"
              >{game.key_value[3]?.value}</th
            >
            <td class="px-6 py-4">{game.key_value[2]?.value}</td>
            <td class="px-6 py-4">{game.key_value[0]?.value}</td>
            <td class="px-6 py-4"
              >{Math.floor((game.stats?.hits / game.stats?.shots) * 100)}%</td
            >
            <td class="px-6 py-4">{game.key_value[5]?.value}</td>
            <td class="px-6 py-4">{game.key_value[6]?.value}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</PrimaryContainer>
