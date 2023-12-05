<script lang="ts">
  import { intlFormatDistance } from "date-fns";
  import type { Schematic, User } from "$lib/types";
  import SchematicCard from "$lib/SchematicCard.svelte";
  import { abbreviateNumber } from "../../../utils";


  let userData: string
  let userSchematics: string
  let uuid: any;

  if (typeof window !== 'undefined') {
    uuid = window.location.pathname.split('/').pop();
  }

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
  class="max-w-6xl mx-auto mt-3ss
w-[calc(100vw-2rem)] justify-between items-left pixel-corners"
>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-left">
    <div
      class="pixel-corners p-3 bg-minecraft-ui-light dark:bg-minecraft-ui-dark"
    >
      <div
        style="--checker-size: 72px; --checker-color: #fff1;"
        class="bg-create-blue pixel-corners w-full h-full p-3 flex gap-1 flex-col bg-checker"
      >
        <div class="flex gap-2">
          <img
            src="https://picsum.photos/500"
            alt=""
            class="w-24 h-24 pixel-corners"
          />
          <div>
            <h2 class="text-2xl font-bold">{user.username}</h2>
            <div>
              <p>
                Joined <b>{intlFormatDistance(user.dateJoined, Date.now())}</b>
              </p>
              <p><b>15</b> Submissions</p>
              <p><b>{abbreviateNumber(user.totalDownloads)}</b> Downloads</p>
            </div>
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <div class="flex gap-2 flex-wrap">
            {#each user.links as link}
              <a class="bg-black dark:bg-white px-2" href={link.url}
                >{link.name}</a
              >
            {/each}
          </div>
          {user.description}
        </div>
      </div>
    </div>
    {#each schematics as schematic}
      <SchematicCard {...schematic} />
    {/each}
  </div>
</main>
