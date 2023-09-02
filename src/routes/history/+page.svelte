<script lang="ts">
  import { history } from "$lib";
  import {
    Pagination,
    PaginationItem,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    TableSearch,
  } from "flowbite-svelte";

  let searchTerm = "";
  let helper = { start: 1, end: 20, total: "X" };

  $: filteredItems = $history.filter(
    (item) =>
      item.key_value[3].value
        .toLowerCase()
        .indexOf(searchTerm.toLowerCase()) !== -1
  );
</script>

<div
  class="w-full h-full p-10 pt-0 pb-3 overflow-y-scroll no-scrollbar flex flex-col gap-3"
>
  <TableSearch
    divClass="[&>*]:p-0 [&>*]:py-2"
    placeholder="search by scenario name"
    bind:inputValue={searchTerm}
  >
    <TableHead>
      <TableHeadCell>NAME</TableHeadCell>
      <TableHeadCell>SCORE</TableHeadCell>
      <TableHeadCell>KILLS</TableHeadCell>
      <TableHeadCell>ACCURACY</TableHeadCell>
      <TableHeadCell>SHOTS TAKEN</TableHeadCell>
      <TableHeadCell>SHOTS HIT</TableHeadCell>
      <TableHeadCell>SENSE</TableHeadCell>
      <TableHeadCell>FOV</TableHeadCell>
    </TableHead>
    <TableBody>
      {#each filteredItems as game}
        <TableBodyRow>
          <TableBodyCell>{game.key_value[3]?.value}</TableBodyCell>
          <TableBodyCell>{game.key_value[2]?.value}</TableBodyCell>
          <TableBodyCell>{game.key_value[0]?.value}</TableBodyCell>
          <TableBodyCell
            >{Math.floor(
              (game.stats?.hits / game.stats?.shots) * 100
            )}%</TableBodyCell
          >
          <TableBodyCell>{game.stats?.shots}</TableBodyCell>
          <TableBodyCell>{game.stats?.hits}</TableBodyCell>
          <TableBodyCell>{game.key_value[5]?.value}</TableBodyCell>
          <TableBodyCell>{game.key_value[6]?.value}</TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </TableSearch>
  <div class="w-full flex items-center justify-between">
    <div class="left-0 text-sm text-gray-700 dark:text-gray-400">
      Showing <span class="font-semibold text-gray-900 dark:text-white"
        >{helper.start}</span
      >
      to
      <span class="font-semibold text-gray-900 dark:text-white"
        >{helper.end}</span
      >
      of
      <span class="font-semibold text-gray-900 dark:text-white"
        >{helper.total}</span
      >
      Entries
    </div>

    <Pagination table>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <span slot="prev" on:click={() => console.log("clicked")}>Prev</span>
      <span slot="next">Next</span>
    </Pagination>
  </div>
</div>
