<script lang="ts">
  import SchematicCard from "$lib/components/schematics/SchematicCard.svelte";
  import SortAndFilter from "$lib/components/SortAndFilter.svelte";
  import { abbreviateNumber } from "$lib/utils";
  import type { components } from "$lib/openapiSchema";

  let collectionId = "";

  const collection = {
    id: "1",
    title: "Train Stations",
    author: {
      id: "1",
      username: "sudolev",
      avatar: "https://picsum.photos/500/800",
    },
    creation_date: new Date(1601111471000),
    thumbnail_url: "https://picsum.photos/500/800",
    schematics: [
      {
        id: "1",
        title: "Very cool schematic",
      },
    ],
  };

  if (typeof window !== "undefined") {
    let collectionIdRaw: string = window.location.pathname
      .replace(/\/$/, "")
      ?.split("/")
      ?.pop()
      ?.split(/[?#]/)[0]!;
    collectionId = collectionIdRaw;
  }

  const schematics: components["schemas"]["FullSchematic"][] = [];
</script>

<svelte:head>
  <title>{collection.title} - Create Schematics</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3ss
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <div
    class=" my-4 px-4 py-3 bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners"
  >
    <h1 class="text-3xl inline">{collection.title}</h1>
    <p class="hidden md:inline text-right float-right">
      {collection.schematics.length} Schematic{collection.schematics.length > 1
        ? "s"
        : ""}
    </p>
    <h2 class="text-xl ml-1 mt-2">
      <img
        class="w-8 h-8 pixel-corners inline"
        src={collection.author.avatar}
        alt="Avatar"
      />
      {collection.author.username}
    </h2>
  </div>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-left">
    {#each schematics as schematic}
      <SchematicCard {schematic} />
    {/each}
  </div>
</main>
