<script lang="ts">
  import { abbreviateNumber } from "$lib/utils";
  import type { components } from "../../openapiSchema";
  export let schematic:
    | components["schemas"]["Schematic"]
    | components["schemas"]["FullSchematic"];
</script>

<a
  href={`/schematic/${schematic.schematic_id}`}
  class="bg-white dark:bg-black/50 mx-auto px-4 py-3 pixel-corners w-full no-default-link"
>
  <h1 class="text-xl font-bold">{schematic.schematic_name}</h1>
  <div class="text-xs text-opacity-50 w-full">
    {#if "author_username" in schematic}
      <p class="inline">
        by <a href={`/user/${schematic.author_username}`} class="underline"
          >{schematic.author_displayname}</a
        >
      </p>
      <p class="inline px-2">|</p>
    {/if}
    <p class="inline"><b>⭳</b> {abbreviateNumber(schematic.downloads)}</p>
    <p class="inline px-2">|</p>
    <!-- <p class="inline text-right">{schematic.toLocaleString("en-US")} UTC</p> -->
    <img
      src={schematic.images[0]}
      alt={schematic.schematic_name}
      class="object-cover pixel-corners w-full h-full overflow-hidden aspect-video my-2"
    />
  </div>
  <ul class="flex flex-wrap gap-2">
    {#if "tags" in schematic}
      {#each schematic.tags as tag}
        <li
          class="text-xs bg-create-blue/50 hover:bg-create-blue/80 px-1 text-opacity-50"
        >
          {tag}
        </li>
      {/each}
    {/if}
  </ul>
</a>
