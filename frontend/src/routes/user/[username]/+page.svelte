<script lang="ts">
  import { intlFormatDistance } from "date-fns";
  import type { Schematic, User } from "$lib";
  import SchematicCard from "$lib/SchematicCard.svelte";

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
    ],
  };

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
      img: "https://picsum.photos/500/800",
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
  <title>{username} - Create Schematics"</title>
</svelte:head>

<main
  class="max-w-6xl mx-auto mt-3ss
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-left">
    <div
      class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners w-full p-3 flex gap-1 flex-col"
    >
      <div class="flex gap-2">
        <img
          src="https://picsum.photos/500"
          alt=""
          class="w-24 h-24 pixel-corners"
        />
        <div>
          <h2 class="text-2xl font-bold">Username123</h2>
          <div class="text-opacity-50 text-black dark:text-white">
            <p>
              Joined <b>{intlFormatDistance(user.dateJoined, Date.now())}</b>
            </p>
            <p><b>15</b> Submissions</p>
            <p><b>24k</b> Downloads</p>
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-1">
        <div class="flex gap-2 flex-wrap">
          {#each user.links as link}
            <a href={link.url}>{link.name}</a>
          {/each}
        </div>
        {user.description}
      </div>
    </div>
    {#each schematics as schematic}
      <SchematicCard {...schematic} />
    {/each}
  </div>
</main>
