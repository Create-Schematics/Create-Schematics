<script lang="ts">
  import { intlFormatDistance } from "date-fns";
  import SchematicCard from "$lib/components/schematics/SchematicCard.svelte";
  import CollectionCard from "$lib/components/collections/CollectionCard.svelte";
  import { abbreviateNumber } from "$lib/utils";
  import { onMount } from "svelte";
  import type { PageData } from "./$types";
  import { page } from "$app/stores";

  export let data: PageData;
  const { user, schematics, collections, currentUser } = data;
  const isPersonalPage = user.user_id == currentUser?.user_id;
</script>

<svelte:head>
  <title>{user.username} - Create Schematics</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <div class="grid grid-cols-1 md:grid-cols-3 md:gap-4 items-left">
    <div class="col-span-1">
      <div
        class="bg-background-dimmed px-3 pt-3 mb-4 pixel-corners"
      >
        <div class="h-18 z-10 relative whitespace-nowrap overflow-visible">
          <img
            src={user.avatar}
            alt=""
            class="ml-2 w-16 h-16 pixel-corners inline"
          />
          <h2
            class="inline ml-2 text-2xl whitespace-nowrap overflow-visible relative"
          >
            {user.username}
          </h2>
        </div>

        <div
          class="bg-white dark:bg-black/50 pixel-corners w-full p-4 pt-5 mb-0 relative z-0 -top-4"
        >
          {#if user.about}
            <p class="w-full text-l max-w-[85%]">
              {user.about}
            </p>
          {/if}
          <hr
            class="my-3 border-minecraft-ui-dark dark:border-minecraft-ui-light"
          />
          <!-- <p class="w-full text-l max-w-[85%]">
            Joined <b>{intlFormatDistance(user.dateJoined, Date.now())}</b>
          </p> -->
          <p class="w-full text-l">
            <b>{schematics.length}</b>
            Submission{#if schematics.length != 1}s{/if}
          </p>
          <!-- <p class="w-full text-l">
            <b>{abbreviateNumber(user.totalDownloads)}</b> Downloads
          </p> -->
          <button
            class="text-minecraft-ui-light hover:text-blue/50 dark:text-minecraft-ui-dark dark:hover:blue/80 cursor-pointer py-1 px-2 m-1 text-2xl pixel-corners absolute top-0 right-0"
          >
            {#if isPersonalPage}
              <p style="transform: scaleX(-1);">✎</p>
            {:else}
              <p class="px-1">⚑</p>
            {/if}
          </button>
          <hr
            class="my-3 border-minecraft-ui-dark dark:border-minecraft-ui-light"
          />
          <div class="w-full grid grid-cols-2 gap-3 mx-auto items-left">
            <!-- {#each user.links as link}
              <a href={link.url} class="no-default-link">
                <button
                  class="bg-blue hover:bg-blue/80 cursor-pointer py-1 pixel-corners w-full pixel-corners"
                >
                  {link.name} 🡕
                </button>
              </a>
            {/each} -->
          </div>
        </div>
      </div>
      <div
        class="bg-background-dimmed pixel-corners w-full pb-1 mb-2 px-3"
      >
        <h2 class="text-2xl pb-1 pt-3"><b>Collections</b></h2>
        {#if collections.length}
          <div class="flex flex-row md:flex-col">
            {#each collections as collection}
              <div class="bg-white dark:bg-black/50 pixel-corners mb-3">
                <CollectionCard {...collection} />
              </div>
            {/each}
          </div>
        {:else}
          <em class=" italic text-black/50 dark:text-white/50"
            >user has no collections</em
          >
        {/if}
      </div>
    </div>

    <!-- Right side -->
    <div class="w-full col-span-2 mx-auto items-left">
      <div
        class="bg-background-dimmed pixel-corners"
      >
        <h2 class="pt-5 text-2xl text-center mb-1">
          <b>Submitted Schematics</b>
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-left p-3 pt-1">
          {#each schematics as schematic}
            <SchematicCard {schematic} />
          {/each}
        </div>
      </div>
    </div>
  </div>
</main>
