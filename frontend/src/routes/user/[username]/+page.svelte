<script lang="ts">
    import { intlFormatDistance} from "date-fns"
    import type { Schematic } from "$lib";
    import SchematicCard from "$lib/SchematicCard.svelte";

    const creationTime = new Date(0);

    const creationRtf = intlFormatDistance(creationTime, Date.now(), {
        locale: "en-US",
    });

    const schematics:Schematic[] = [
        {
            tags: ["farm", "Above & Beyond", "trains", "equipment", "steampunk", "novel"],
            uploadDate: new Date(1701111471000),
            title: "Very cool schematic",
            img: "https://picsum.photos/500/800",
            downloads: 603,
            likes: 60,
            dislikes: 40,
            views: 894,
            author: "Szedann",
            id: "1"
        }
    ]
</script>
<main class="max-w-6xl mx-auto mt-3ss
w-[calc(100vw-2rem)] justify-between items-left pixel-corners">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-left">
        <div class="col-span-1">
            <div class="h-24 z-10 relative whitespace-nowrap overflow-visible">
                <img src="https://picsum.photos/500" alt="" class="ml-5 w-24 h-24 pixel-corners inline"/>
                <h2 class="inline ml-3 text-2xl whitespace-nowrap overflow-visible">Username123</h2>
            </div>

            <div class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners w-full p-6 relative z-0 -top-4">
                <p class="w-full text-xl max-w-[85%]">
                    Joined <b>{creationRtf}</b>
                </p>
                <p class="w-full text-xl">
                    <b>15</b> Submissions
                </p>
                <p class="w-full text-xl">
                    <b>24k</b> Downloads
                </p>
                <button class="bg-create-blue/50 hover:bg-create-blue/80 cursor-pointer py-1 px-3 m-5 pixel-corners absolute top-0 right-0" >⚑</button>
            <hr class="my-3">
                <div class="w-full grid grid-cols-2 gap-3 mx-auto items-left">
                    <button class="bg-create-blue/50 hover:bg-create-blue/80 cursor-pointer py-1 px-3pixel-corners w-full pixel-corners">Modrinth 🡕</button>
                    <button class="bg-create-blue/50 hover:bg-create-blue/80 cursor-pointer py-1 px-3pixel-corners w-full pixel-corners">Github 🡕</button>
                </div>
            </div>

        </div>

        <!-- Right side -->
        <div class="w-full col-span-2 mx-auto mt-10 items-left">
            <h2 class="text-3xl text-center mb-1">Submitted Schematics</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-left">
                {#each schematics as schematic}
                    <SchematicCard {...schematic}/>
                {/each}
            </div>
        </div>
    </div>
</main>