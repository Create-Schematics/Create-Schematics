<script lang="ts">
  import Slider from "$lib/Slider.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
  const { schematic, tags, comments } = data;

  let selectedOption = "description";

  function handleOptionClick(option: string) {
    selectedOption = option;
  }
</script>

<svelte:head>
  <title>{schematic} - Create Schematics</title>
</svelte:head>

<main class="max-w-6xl w-[calc(100vw-2rem)] flex flex-col mx-auto gap-3">
  <section class="flex gap-3 w-full justify-stretch flex-wrap md:flex-nowrap">
    <div
      class="flex md:w-2/3 bg-minecraft-ui-light dark:bg-minecraft-ui-dark p-3 pixel-corners"
    >
      <Slider
        images={[
          "https://picsum.photos/700/500",
          "https://picsum.photos/700/500",
          "https://picsum.photos/700/500",
        ]}
      />
    </div>
    <div
      class="flex flex-col gap-2 w-full md:w-1/3 justify-between bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners"
    >
      <div class="flex flex-col gap-2">
        <div class="p-3 pb-0">
          <h1 class="text-2xl font-bold">{schematic.schematic_name}</h1>
          <h3 class="text-xs text-opacity-50">
            by <a href={`/user/${schematic.author}`} class="underline"
              >{schematic.author}</a
            >
          </h3>
        </div>
        <div
          class="px-2 text-sm pixel-corners p-2 flex bg-white dark:bg-black m-2 divide-x divide-create-blue/20"
        >
          <div class="flex flex-col items-center w-full">
            <h1 class="text-xl">{schematic.downloads}</h1>
            <span class="text-xs">Downloads</span>
          </div>
          <div class="flex flex-col items-center w-full">
            <h1 class="text-xl">60%</h1>
            <span class="text-xs">Likes</span>
          </div>
          <div class="flex flex-col items-center w-full">
            <h1 class="text-xl">894</h1>
            <span class="text-xs">Views</span>
          </div>
        </div>
        <ul class="flex flex-wrap gap-2 px-2">
          {#each tags as tag}
            <li
              class="text-xs bg-create-blue/80 dark:bg-create-blue/20 px-1 text-opacity-50"
            >
              {tag}
            </li>
          {/each}
        </ul>
        <div class="w-full p-3 bg-create-blue/10 dark:bg-black/20">
          <h2>Required mods:</h2>
          <ul
            class="flex flex-col gap-2 overflow-y-scroll max-h-64 p-2 bg-fixed no-scrollbar"
          >
            <!-- {#each mods as mod}
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
            {/each} -->
          </ul>
        </div>
      </div>
      <div class="flex flex-col gap-3 p-3 pt-0">
        <a
          href="create-schem://123124"
          style="--checker-color: #fff1"
          class="w-full no-default-link bg-create-blue text-xl hover:text-slate-100 font-bold p-2 text-center pixel-corners bg-checker text-white"
          >Download</a
        >
      </div>
    </div>
  </section>
  <section
    class=" bg-minecraft-ui-light dark:bg-minecraft-ui-dark p-3 pixel-corners"
  >
    <nav class="mt-3 flex gap-2">
      <button
        class="text-xl pixel-top px-5 py-2 relative top-1 cursor-pointer {selectedOption ===
        'description'
          ? 'bg-white dark:bg-black/30'
          : 'dark:bg-create-blue-dark'}"
        on:click={() => handleOptionClick("description")}>Description</button
      >
      <button
        class="text-xl pixel-top px-5 py-2 relative top-1 cursor-pointer {selectedOption ===
        'details'
          ? 'bg-white dark:bg-black/30'
          : 'dark:bg-create-blue-dark'}"
        on:click={() => handleOptionClick("details")}>Details</button
      >
      <button
        class="text-xl pixel-top px-5 py-2 relative top-1 cursor-pointer {selectedOption ===
        'comments'
          ? 'bg-white dark:bg-black/30'
          : 'dark:bg-create-blue-dark'}"
        on:click={() => handleOptionClick("comments")}>Comments</button
      >
    </nav>
    {#if selectedOption === "description"}
      <pre class="bg-white dark:bg-black/30 p-4 pixel-corners">
       {schematic.body} 
      </pre>
    {/if}
    {#if selectedOption === "details"}
      <div class="bg-white dark:bg-black/30 p-4 pixel-corners">
        details, whatever those are
      </div>
    {/if}
    {#if selectedOption === "comments"}
      <div class="">
        <div class="p-4 mb-4 bg-white dark:bg-black/30 pixel-corners">
          <textarea
            class="w-full h-auto outline-none bg-transparent"
            placeholder={comments.length == 0
              ? "Write the first comment!"
              : "Contribute to the discussion!"}
          ></textarea>
          <button class="bg-create-blue text-white px-4 py-1 pixel-corners"
            >Post</button
          >
        </div>
        {#each comments as comment}
          <div class="p-4 mb-2 bg-white dark:bg-black/30 pixel-corners">
            {comment.comment_body}
          </div>
        {/each}
      </div>
    {/if}
  </section>
</main>
