<script lang="ts">
  import { abbreviateNumber } from "$lib/utils";
  import type { components } from "$lib/openapiSchema";

  let isUserPage = false;
  if (typeof window !== "undefined") {
    isUserPage = window.location.href.includes("user");
  }
  export let {
    collection_id,
    collection_name,
    user_id,
    is_private,
  }: components["schemas"]["Collection"] = {
    collection_id: "",
    collection_name: "",
    is_private: 1 == 1,
    user_id: "",
  };
</script>

<a
  href={`../../user/${user_id}/collection/${collection_id}`}
  class="block mx-auto px-3 py-3 pixel-corners w-full no-default-link"
>
  <h1 class="text-xl font-bold leading-5">{collection_name}</h1>
  <!-- <div class="text-xs text-opacity-50 w-full pb-1">
    {#if !isUserPage}
      <p class="inline">
        by <a href={`/user/${author}`} class="underline">{author}</a>
      </p>
      <p class="inline px-2">|</p>
    {/if}
    <p class="inline text-right">{views} Views</p>
  </div> -->
  <div class="overflow-x-scroll w-full">
    <ul
      class="flex gap-2 overflow-x-scroll w-full flex-nowrap whitespace-nowrap"
    >
      <!-- {#each slicedTags as tag}
        <li
          class="text-xs bg-blue hover:bg-blue/80 px-1 text-opacity-50 whitespace-nowrap"
        >
          {tag}
        </li>
      {/each} -->
    </ul>
  </div>
</a>
