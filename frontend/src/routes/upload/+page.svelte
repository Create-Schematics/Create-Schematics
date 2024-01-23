<script lang="ts">
  import Slider from "$lib/components/Slider.svelte";
  import { getSchematicData } from "$lib/schematics";
  let files: FileList;
  let schematicData: Awaited<ReturnType<typeof getSchematicData>>;
  $: if (files)
    files[0].arrayBuffer().then(async (arrayBuffer) => {
      schematicData = await getSchematicData(Buffer.from(arrayBuffer));
    });
</script>

<svelte:head>
  <title>Upload - Create Schematics</title>
</svelte:head>

<main class="max-w-4xl w-[calc(100vw-2em)] flex flex-col gap-4 items-center">
  <div
    class="bg-background-dimmed p-4 pixel-corners flex flex-col items-center gap-4"
  >
    <div class="flex justify-center gap-2 w-full">
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
      <div
        class="w-16 h-16 bg-gray-500 pixel-corners text-3xl flex items-center justify-center"
      >
        /
      </div>
    </div>
  </div>
  <div
    class="bg-background p-4 pixel-corners flex flex-col items-center gap-4 w-full h-96"
  >
    <h1 class="text-xl">Upload the file</h1>
    <div class="h-full flex flex-col gap-2 w-full items-center">
      <label
        class="bg-blue px-2 py-1 text-lg pixel-corners block cursor-pointer"
      >
        Upload
        <input class="hidden" type="file" accept=".nbt" bind:files />
      </label>
      <div class="bg-black h-full w-full pixel-corners text-white">
        {#if files}
          {#if schematicData}
            <ul>
              {#each schematicData.blockList.entries() as block}
                <li>
                  {block[1]}x {block[0]}
                </li>
              {/each}
            </ul>
            <ul>
              {#each schematicData.mods.keys() as mod}
                <li>{mod}</li>
              {/each}
            </ul>
          {/if}
        {/if}
      </div>
    </div>
    <div class="flex justify-between w-full">
      <div></div>
      <input
        class="pixel-corners bg-blue px-2 py-1 text-lg"
        type="button"
        value="next"
      />
    </div>
  </div>
</main>
