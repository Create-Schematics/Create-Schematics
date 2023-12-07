<script lang="ts">
  import { intlFormatDistance } from "date-fns";
  import type { Schematic, User, Collection } from "$lib/types";
  import SchematicCard from "$lib/SchematicCard.svelte";
  import CollectionCard from "$lib/CollectionCard.svelte";
  import { abbreviateNumber } from "../../../utils";
  import { onMount } from 'svelte';

  let userData: string
  let userSchematics: string
  let uuid: string|undefined;
  const isPersonalPage = true;

  if (typeof window !== 'undefined') {
    uuid = window.location.pathname.replace(/\/$/, '')?.split('/')?.pop()?.split(/[?#]/)[0];
    if (typeof uuid === undefined || uuid === "" || uuid === null) {
      uuid = "-1"
    }
  }

  let isDesktop = false;

  onMount(() => {
    const mediaQuery = window.matchMedia('(min-width: 768px)');
    isDesktop = mediaQuery.matches;

    const handleResize = () => {
      isDesktop = mediaQuery.matches;
    };

    mediaQuery.addEventListener('change', handleResize);

    return () => {
      mediaQuery.removeEventListener('change', handleResize);
    };
  });


  async function getUserData () {
		const res = await fetch('https://httpbin.org/json', {
			method: 'GET'
		})
		
		const json = await res.json()
		userData = JSON.stringify(json)
    console.log(userData)
	}
  async function getUserSchematics () {
		const res = await fetch(`/api/v1/users/${uuid}}/schematics`, {
			method: 'GET'
		})
		
		const json = await res.json()
		userSchematics = JSON.stringify(json)
    console.log(userSchematics)
	}


  const user: User = {
    username: "Username123",
    dateJoined: new Date(1601111471000),
    totalDownloads: 24000,
    avatar: "https://picsum.photos/500",
    id: "1",
    description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    links: [
      {
        name: "Twitter",
        url: "https://twitter.com",
      },
      {
        name: "Youtube",
        url: "https://youtube.com",
      },
      {
        name: "Modrinth",
        url: "https://modrinth.com",
      },
      {
        name: "GitHub",
        url: "https://github.com",
      },
    ],
  };

  const collections: Collection[] = [{
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
    likes: 60,
    dislikes: 40,
    views: 894,
    author: "Szedann",
    id: "1",
    schematicIds: ["balls"]
  }];
  collections.push(...collections);
  collections.push(...collections);
  
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
      uploadDate: new Date(1601111471000),
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
  schematics.push(...schematics);
  schematics.push(...schematics);
  schematics.push(...schematics);
</script>

<svelte:head>
  <title>{user.username} - Create Schematics"</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
<div class="grid grid-cols-1 md:grid-cols-3 md:gap-4 items-left">
  <div class="col-span-1">
    <div class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark px-3 pt-4 mb-4 pixel-corners">
      <div class="h-18 z-10 relative whitespace-nowrap overflow-visible">
          <img src="https://picsum.photos/500" alt="" class="ml-2 w-16 h-16 pixel-corners inline"/>
          <h2 class="inline ml-2 text-2xl whitespace-nowrap overflow-visible relative ">{user.username}</h2>
      </div>

      <div class="bg-white dark:bg-black/50 pixel-corners w-full p-4 pt-5 mb-0 relative z-0 -top-4">
          <p class="w-full text-l max-w-[85%]">
            {user.description}
          </p>
        <hr class="my-3">
          <p class="w-full text-l max-w-[85%]">
            Joined <b>{intlFormatDistance(user.dateJoined, Date.now())}</b>
          </p>
          <p class="w-full text-l">
              <b>{schematics.length}</b> Submission{#if schematics.length > 1}s{/if}
          </p>
          <p class="w-full text-l">
              <b>{abbreviateNumber(user.totalDownloads)}</b> Downloads
          </p>
        <button class="text-minecraft-ui-light hover:text-create-blue/50 dark:text-minecraft-ui-dark dark:hover:create-blue/80 cursor-pointer py-1 px-2 m-1 text-2xl pixel-corners absolute top-0 right-0" >
          {#if isPersonalPage}
            <p style="transform: scaleX(-1);">✎</p>
          {:else}
            <p class="px-1">⚑</p>
          {/if}
          </button>
      <hr class="my-3">
          <div class="w-full grid grid-cols-2 gap-3 mx-auto items-left">
            {#each user.links as link}
              <a href="{link.url}" class="no-default-link">
                <button class="bg-create-blue/50 hover:bg-create-blue/80 cursor-pointer py-1 pixel-corners w-full pixel-corners">
                  {link.name} 🡕
                </button>
              </a>
            {/each}
          </div>
      </div>
    </div>
      <div class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners w-full pb-1 mb-4">
        {#if isDesktop}
          <h2 class="text-2xl px-4 pt-4">Collections</h2>
          {#each collections as collection}
            <div class="bg-white dark:bg-black/50 pixel-corners m-3"><CollectionCard {...collection} /></div>
          {/each}
        {:else}
          <h2 class="text-xl px-4 pt-3 pb-2"><b>Collections 🡕</b></h2>
        {/if}

      </div>
  </div>

  <!-- Right side -->
  <div class="w-full col-span-2 mx-auto items-left">
    <div class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners">
      <h2 class="pt-4 text-2xl text-center mb-1">Submitted Schematics</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-left p-3 ">
        {#each schematics as schematic}
          <SchematicCard {...schematic}/>
        {/each}
      </div>
  </div>
</div>
</div></main>
