<script lang="ts">
  import SchematicCard from "$lib/components/schematics/SchematicCard.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
  const { schematics } = data;
  console.log(schematics);
</script>

<svelte:head>
  <title>Browse - Create Schematics</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <h2 class="text-2xl text-white/500 p-1">Browse Schematics</h2>
  <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mx-auto items-left">
    {#each schematics as schematic}
      <SchematicCard {schematic} />
    {/each}
  </div>
</main>
