<script lang="ts">
  import type { Schematic } from "$lib/types";
  import SchematicCard from "$lib/SchematicCard.svelte";

  const schematics: Schematic[] = [
    {
      tags: [
        "farm",
        "Above & Beyond",
        "trains",
        "equipment",
        "steampunk",
        "novel",
      ],
      uploadDate: new Date(1701111471000),
      title: "Very cool schematic",
      images: ["https://picsum.photos/500/800"],
      downloads: 603,
      likes: 60,
      dislikes: 40,
      views: 894,
      author: "Szedann",
      id: "1",
    },
  ];
</script>

<svelte:head>
  <title>Browse - Create Schematics"</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <h2 class="text-2xl text-white/500 p-1">Browse Schematics</h2>
  <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mx-auto items-left">
    {#each schematics as schematic}
      <SchematicCard {...schematic} />
    {/each}
  </div>
</main>
