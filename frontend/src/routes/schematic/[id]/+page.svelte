<script lang="ts">
  import Slider from "$lib/Slider.svelte";

  const tags = ["farm", "Above & Beyond"];
  const mods = ["Create: Steam 'n' Rails", "Create: Extended Bogeys"];
</script>

<main
  class="max-w-6xl bg-create-blue/5 rounded-2xl w-[calc(100vw-2rem)] p-4 flex flex-col mx-auto ring-1 ring-create-blue/20 gap-3 divide-y divide-create-blue/30"
>
  <section class="flex gap-3 w-full justify-stretch flex-wrap md:flex-nowrap">
    <div class="flex md:w-2/3 aspect-video">
      <Slider
        images={[
          "https://picsum.photos/800/450",
          "https://picsum.photos/800/450",
          "https://picsum.photos/800/450",
        ]}
      />
    </div>
    <div class="flex flex-col gap-2 w-full md:w-1/3 justify-between">
      <div class="flex flex-col gap-2">
        <div>
          <h1 class=" text-2xl font-bold">Very cool schematic</h1>
          <h3 class="font-mono text-xs text-white/50">
            by <a href="/user/Szedann" class="underline">Szedann</a>
          </h3>
        </div>
        <ul class="flex flex-wrap gap-2">
          {#each tags as tag}
            <li
              class="font-mono text-xs bg-create-blue/20 px-1 ring-1 ring-create-blue/40 rounded-md text-white/50"
            >
              {tag}
            </li>
          {/each}
        </ul>
        <div class="w-full">
          <h2>Required mods:</h2>
          <ul class="flex flex-col gap-2">
            {#each mods as mod}
              <li
                class="flex gap-2 bg-black/30 rounded-xl w-full p-2 ring-1 ring-create-blue/10"
              >
                <img src="https://picsum.photos/40" alt="" class="rounded-lg" />
                <div>
                  <h2 class="font-bold">{mod}</h2>
                  <div class="text-xs flex gap-1 font-mono">
                    <a href="https://modrinth.com">Modrinth</a>
                    <a href="https://cursefurge.com">CurseForge</a>
                  </div>
                </div>
              </li>
            {/each}
          </ul>
        </div>
      </div>
      <div class="flex flex-col gap-3">
        <a
          href="create-schem://123124"
          class="w-full bg-green-500/30 rounded-lg p-2 text-center ring-green-500/50 ring-1 text-white/80"
          >Download</a
        >
        <a
          href="create-schem://123124"
          class="w-full bg-create-blue/30 rounded-lg p-2 text-center ring-create-blue/50 ring-1 text-white/80"
          >Build</a
        >
      </div>
    </div>
  </section>
  <section>
    <h1 class="text-2xl font-bold my-3">Description</h1>
    <div class="bg-black/50 p-4 rounded-xl">
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem
      blanditiis dolores quaerat iste veritatis. Sequi saepe, ut aut voluptatem
      enim rerum! Debitis praesentium error aliquam. Deserunt doloremque numquam
      eligendi alias sit explicabo. Laboriosam cum dolor eaque voluptas id
      quisquam optio iste maxime, totam dicta libero reprehenderit facilis eum
      quaerat magni aliquam, commodi, nam corporis. Ratione impedit repellat
      temporibus maiores ad? Quis quam necessitatibus, tempora incidunt fugiat
      perspiciatis quibusdam ipsam suscipit recusandae veniam modi quaerat,
      debitis, qui dolorum! Deleniti nihil non ut molestiae facere maxime
      officia error praesentium, eveniet doloribus, impedit blanditiis autem
      amet voluptatibus atque dolor enim? Animi ullam minus aliquam quaerat
      adipisci earum maxime magnam odio ducimus illum soluta iure dolores
      asperiores consequuntur numquam consectetur incidunt, impedit ex officia
      aut deleniti vitae dolore neque doloremque. Odio architecto vitae, iure,
      aliquam quia expedita consequuntur quam accusamus magnam ipsum
      repellendus. Beatae facilis iste, odit repellendus voluptates nemo dolorum
      error. Sed praesentium quo eos, saepe voluptatibus doloremque repellendus
      asperiores odit vel maiores. Reiciendis autem amet earum, nostrum eius
      suscipit, deserunt voluptatum nisi unde dignissimos maxime eum similique
      sunt repellendus voluptatibus quasi culpa commodi quibusdam veniam, animi
      cum. Quos sit aperiam, expedita consequuntur, tenetur nostrum dolorem
      nihil commodi ipsa officiis quasi obcaecati! In.
    </div>
  </section>
</main>
