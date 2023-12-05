<script lang="ts">
  import { intlFormatDistance } from "date-fns";
  import type { Schematic, User } from "$lib/types";
  import SchematicCard from "$lib/SchematicCard.svelte";
  import { abbreviateNumber } from "../../../../../utils";

  let userFavorites: string

  async function getUserFavorites () {
		const res = await fetch(`/api/v1/schematics/favorites`, {
			method: 'GET'
		})
		
		const json = await res.json()
		userFavorites = JSON.stringify(json)
    console.log(userFavorites)
	}

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
  <title>Favorites - Create Schematics"</title>
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
            <h2 class="text-2xl font-bold"></h2>
            <div>
              <p>
              </p>
              <p><b>15</b> Submissions</p>
            </div>
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <div class="flex gap-2 flex-wrap">
          </div>
        </div>
      </div>
    </div>
    {#each schematics as schematic}
      <SchematicCard {...schematic} />
    {/each}
  </div>
</main>
