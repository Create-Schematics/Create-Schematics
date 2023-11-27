<script lang="ts">
    const tags = ["farm", "Above & Beyond", "trains", "equipment", "steampunk", "novel"];
    const mods = ["Create: Steam 'n' Rails", "Create: Extended Bogeys", "aa", "bb", "cc"];
    const uploadDate = 1701111471000;
    var lang = 'en-US';
  </script>

<main class="max-w-6xl p-4 mx-auto mt-3
w-[calc(100vw-2rem)] justify-between items-left pixel-corners">
<h2 class="text-2xl text-white/500 p-1">Browse Schematics</h2>
<div class="grid grid-cols-2 md:grid-cols-3 gap-4 mx-auto items-left">
    <div class=" mx-auto px-4 py-3 bg-minecraft-ui-dark pixel-corners w-full">
        <h1 class="text-xl font-bold">Very cool schematic</h1>
        <div class="text-xs text-white/50 w-full">
            <p class="inline">by <a href="/user/Szedann" class="underline">Szedann</a></p>
            <p class="inline px-2">|</p>
            <p class="inline text-right">{new Date(uploadDate).toLocaleString(lang, { timeZone: 'UTC' })} UTC</p>        </div>
        <div class="w-full py-3">
            <img src="https://picsum.photos/500/300" alt="" class="object-cover pixel-corners" />
        </div>
        <div class="overflow-x-scroll whitespace-nowrap scrollbar-x scrollbar-small">
            <ul class="flex flex-nowrap gap-2 mb-3">
                {#each tags as tag}
                    <li class="text-xs bg-create-blue/20 px-1 text-white/50">
                        {tag}
                    </li>
                {/each}
            </ul>
        </div>  
    </div>
</div>
</main>