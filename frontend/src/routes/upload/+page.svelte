<script lang="ts">
  import Slider from "$lib/Slider.svelte";
  import type { Schematic, SchematicDetails } from "$lib";

  let files: FileList | null = null;

  $: files,
    (schematic.images = files
      ? [...files].map((file) => URL.createObjectURL(file))
      : []);

  const schematic: Partial<SchematicDetails> = {
    tags: [],
    title: "Untitled schematic",
    images: [],
    author: "Szedann",
    id: "1",
  };
</script>

<svelte:head>
  <title>{schematic.title} - Create Schematics</title>
</svelte:head>

<main class="max-w-6xl w-[calc(100vw-2rem)] flex flex-col mx-auto gap-3">
  <section class="flex gap-3 w-full justify-stretch flex-wrap md:flex-nowrap">
    <div
      class="flex md:w-2/3 bg-minecraft-ui-light dark:bg-minecraft-ui-dark p-3 pixel-corners"
    >
      {#if schematic.images && schematic.images.length > 0}
        <Slider images={schematic.images} />
      {:else}
        <div
          class="w-full aspect-video bg-slate-500 pixel-corners flex items-center justify-center flex-col"
        >
          <h3>Upload images</h3>
          <input type="file" multiple accept="image/*" bind:files />
        </div>
      {/if}
    </div>
    <div
      class="flex flex-col gap-2 w-full md:w-1/3 justify-between bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners"
    >
      <div class="flex flex-col gap-2">
        <div class="p-3 pb-0">
          <h1 class="text-2xl font-bold">
            <input
              type="text"
              class="bg-black/20 outline-none p-1"
              bind:value={schematic.title}
            />
          </h1>
        </div>
        <ul class="flex flex-wrap gap-2 px-2">
          {#each schematic.tags ?? [] as tag, i}
            <input
              type="button"
              class="text-xs bg-create-blue/80 dark:bg-create-blue/20 px-1 text-opacity-50 cursor-pointer hover:bg-red-600 hover:line-through"
              value={tag}
              on:click={() => {
                schematic.tags?.splice(i, 1);
                schematic.tags = [...(schematic.tags ?? [])];
              }}
            />
          {/each}
          <input
            type="text"
            class="text-xs bg-create-blue px-1 text-opacity-50 outline-none w-24"
            placeholder="add tag"
            on:keydown={(e) => {
              if (e.key != "Enter") return;
              schematic.tags = [
                ...(schematic.tags ?? []),
                e.currentTarget.value,
              ];
              e.currentTarget.value = "";
            }}
          />
        </ul>
        <div class="w-full p-3 bg-create-blue/10 dark:bg-black/20">
          <h2>Required mods:</h2>
          <ul
            class="flex flex-col gap-2 overflow-y-scroll max-h-64 p-2 bg-fixed no-scrollbar"
          >
            {#each schematic.mods ?? [] as mod}
              <li
                class="flex gap-2 bg-white dark:bg-black/30 w-full p-2 pixel-corners bg-checker"
                style="--checker-color: #0001;"
              >
                <img
                  src="https://picsum.photos/40"
                  alt=""
                  class="pixel-corners h-12 w-12"
                />
                <div>
                  <h2 class="font-bold">{mod}</h2>
                  <div class="text-xs flex gap-1">
                    <a href="https://modrinth.com">Modrinth</a>
                    <a href="https://cursefurge.com">CurseForge</a>
                  </div>
                </div>
              </li>
            {/each}
          </ul>
        </div>
      </div>
    </div>
  </section>
  <section
    class=" bg-minecraft-ui-light dark:bg-minecraft-ui-dark p-3 pixel-corners"
  >
    <nav class="my-3 flex gap-2">
      <h2 class="text-xl underline">Description</h2>
      <h2 class="text-xl text-opacity-20">Details</h2>
    </nav>
    <div class="bg-white dark:bg-black/50 p-4 pixel-corners">
      <textarea class="w-full h-full bg-black/10 outline-none"></textarea>
    </div>
  </section>
  <section
    class=" bg-minecraft-ui-light dark:bg-minecraft-ui-dark p-3 pixel-corners text-right"
  >
    <input
      type="button"
      value="Submit"
      class="bg-create-blue p-2 pixel-corners text-2xl cursor-pointer"
    />
  </section>
</main>
