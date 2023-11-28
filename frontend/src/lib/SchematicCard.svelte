<script lang="ts">
  import type { Schematic } from "$lib";

  export const { tags, uploadDate, title, author, downloads, id }: Schematic = {
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
  };
  const slicedTags = tags.slice(0, 3);
</script>

<a
  href={`/schematic/${id}`}
  class=" mx-auto px-4 py-3 bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners w-full no-default-link"
>
  <h1 class="text-xl font-bold">{title}</h1>
  <div class="text-xs text-opacity-50 w-full">
    <p class="inline"><b>⭳</b>{downloads}</p>
    <p class="inline">
      by <a href={`/user/${author}`} class="underline">{author}</a>
    </p>
    <p class="inline px-2">|</p>
    <p class="inline text-right">{uploadDate.toLocaleString("en-US")} UTC</p>
    <img
      src="https://picsum.photos/500/800"
      alt=""
      class="object-cover pixel-corners w-full h-full overflow-hidden aspect-video my-2"
    />
  </div>
  <ul class="flex flex-wrap gap-2">
    {#each slicedTags as tag}
      <li
        class="text-xs bg-create-blue/80 dark:bg-create-blue/20 px-1 text-opacity-50"
      >
        {tag}
      </li>
    {/each}
  </ul>
</a>
