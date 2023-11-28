<script lang="ts">
    const tags = ["farm", "Above & Beyond", "trains", "equipment", "steampunk", "novel"];
    const mods = ["Create: Steam 'n' Rails", "Create: Extended Bogeys", "aa", "bb", "cc"];
    const uploadDate = 1701111471000;
    var lang = 'en-US';

    const creationTime = 1701133584000;

    function getAccountAge(accountCreationTimestamp: number): string {
        const timeDifference = Date.now() - accountCreationTimestamp;

        const seconds = Math.floor(timeDifference / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);
        const days = Math.floor(hours / 24);
        const months = Math.floor(days / 30);
        const years = Math.floor(months / 12);

        if (years > 0) {
            return `${years} year${years > 1 ? 's' : ''}`;
        } else if (months > 0) {
            return `${months} month${months > 1 ? 's' : ''}`;
        } else if (days > 0) {
            return `${days} day${days > 1 ? 's' : ''}`;
        } else if (hours > 0) {
            return `${hours} hour${hours > 1 ? 's' : ''}`;
        } else if (minutes > 0) {
            return `${minutes} minute${minutes > 1 ? 's' : ''}`;
        } else {
            return `${seconds} second${seconds !== 1 ? 's' : ''}`;
        }
    }
</script>
<main class="max-w-6xl mx-auto mt-3ss
w-[calc(100vw-2rem)] justify-between items-left pixel-corners">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-left">
        <div class="col-span-1">
            <div class="h-24 z-10 relative whitespace-nowrap overflow-visible">
                <img src="https://picsum.photos/500" alt="" class="ml-5 w-24 h-24 pixel-corners inline"/>
                <h2 class="inline ml-3 text-2xl whitespace-nowrap overflow-visible">Username123</h2>
            </div>

            <div class="bg-minecraft-ui-dark pixel-corners w-full p-6 relative z-0 -top-4">
                <p class="w-full text-xl max-w-[85%]">
                    Joined <b>{getAccountAge(creationTime)}</b> ago
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

                <div class="mx-auto px-4 py-3 bg-minecraft-ui-dark pixel-corners w-full">
                    <h1 class="text-xl font-bold">Very cool schematic</h1>
                    <div class="text-xs text-white/50 w-full">
                        <p class="inline"><b>⭳</b> 5,152</p>
                        <p class="inline px-2">|</p>
                        <p class="inline text-right">{new Date(uploadDate).toLocaleString(lang, { timeZone: 'UTC' })}</p>        </div>
                        <div class="w-full py-3 aspect-video">
                            <img src="https://picsum.photos/500/300" alt="" class="object-cover pixel-corners w-full h-full overflow-hidden" />
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

                <div class=" mx-auto px-4 pt-3 bg-minecraft-ui-dark pixel-corners w-full">
                    <h1 class="text-xl font-bold">mega cool schematic</h1>
                    <div class="text-xs text-white/50 w-full">
                        <p class="inline"><b>⭳</b> 2,121</p>
                        <p class="inline px-2">|</p>
                        <p class="inline text-right">{new Date(uploadDate - 5000000000).toLocaleString(lang, { timeZone: 'UTC' })} UTC</p>        </div>
                    <div class="w-full py-3 aspect-video">
                        <img src="https://picsum.photos/501/300" alt="" class="object-cover pixel-corners w-full h-full" />
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
        </div>
    </div>
</main>