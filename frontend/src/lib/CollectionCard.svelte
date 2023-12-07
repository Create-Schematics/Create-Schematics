<script lang="ts">
  import type { Collection } from "$lib/types";
  import { abbreviateNumber } from "../utils";

  let isUserPage = false;
  if (typeof window !== 'undefined') {
    isUserPage = window.location.href.includes("user");
  }
  export const { id, tags, creationDate, title, likes, dislikes, views, author, schematicIds }: Collection = {
    id: "1",
    tags: [
      "farm",
      "Above & Beyond",
      "trains",
      "equipment",
      "steampunk",
      "novel",
    ],
    creationDate: new Date(1701111471000),
    title: "Favorites",
    views: 894,
    likes: 60,
    dislikes: 40,
    author: "Szedann",
    schematicIds: ["balls"]
  };
  const slicedTags = tags.slice(0, 3);
</script>

<a
href={`../../user/${author}/collection/${id}`}
class="block mx-auto px-3 py-3 pixel-corners w-full no-default-link"
>
  <h1 class="text-xl font-bold leading-5">{title}</h1>
  <div class="text-xs text-opacity-50 w-full pb-1">
    {#if !isUserPage}
      <p class="inline">
        by <a href={`/user/${author}`} class="underline">{author}</a>
      </p>
      <p class="inline px-2">|</p>
    {/if}
    <p class="inline text-right">{views} Views</p>
  </div>
  <div class="overflow-x-scroll w-full">
    <ul class="flex gap-2 overflow-x-scroll w-full flex-nowrap whitespace-nowrap">
      {#each slicedTags as tag}
        <li
          class="text-xs bg-create-blue/80 dark:bg-create-blue/20 px-1 text-opacity-50 whitespace-nowrap"
        >
          {tag}
        </li>
      {/each}
    </ul>
  </div>
</a>
