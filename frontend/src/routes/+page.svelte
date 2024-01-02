<script>
    import InView from "$lib/components/layout/InView.svelte";

    import { fly } from "svelte/transition";
</script>

<svelte:head>
    <title>Create Schematics</title>
</svelte:head>

<main class="flex flex-col">
    <div class="bg-create-blue text-center p-12 relative h-[40vh]">
        <h1 class="text-6xl font-bold">Create Schematics</h1>
        <p class="mt-4 text-xl">The best way to search, share and download schematics.</p>

        <div class="absolute w-16 h-full bg-slate-100 opacity-30 top-0 left-[20%]"/>
        <div class="absolute h-16 w-full bg-slate-100 opacity-30 left-0 top-[4rem]"/>

        <div class="my-20">
            <a href="../browse" class="no-default-link"><button class="bg-checker px-4 py-2 text-xl font-bold bg-create-blue hover:bg-create-blue/70 text-white rounded-md pixel-corners">Browse Schematics</button></a>
        </div>
    </div>

    <div id="schematics" class="p-12">
        <InView>
            <div 
                class="flex flex-col md:flex-row justify-around gap-12 max-w-6xl m-auto"
                in:fly={{duration: 700, delay: 100, y: 60}}
            >
                <div class="text-right flex justify-end m-auto">
                    <img alt="table" src="assets/table.png" class="max-h-[440px]">
                </div>
                <div class="flex flex-col justify-center items-center m-auto">
                    <h2 class="text-4xl font-bold">Thousands&nbsp;of Schematics</h2>
                    <p class="mt-2 text-2xl">All&nbsp;within&nbsp;arms&nbsp;reach, all&nbsp;for&nbsp;free.&nbsp;</p>
                </div>
            </div>
        </InView>
    </div>

    <div class="p-12 bg-minecraft-ui-light dark:bg-minecraft-ui-dark text-center">
        <InView>
            <div 
                class="flex flex-col-reverse md:flex-row justify-around gap-12 max-w-6xl m-auto"
                in:fly={{duration: 700, delay: 300, y:60}}
            >
                <div class="flex flex-col justify-center items-center m-auto">
                    <h2 class="text-4xl font-bold">Modern, Stable<br/>and Easy to use</h2>
                    <p class="mt-2 text-2xl">Open source and libre.</p>
                </div>
                <div class="flex justify-end p-2 m-auto">
                    <img alt="arm" src="assets/arm.png" class="max-h-[440px]">
                </div>
            </div>
        </InView>
    </div>
    <div class="md:w-[calc(100vw-2rem)] max-w-6xl md:px-8 py-8 mt-10 mb-32 mx-auto bg-checker pixel-corners">
        <h2 class="text-4xl font-bold w-full text-center px-8">What&nbsp;are&nbsp;you waiting&nbsp;for?</h2>
        <div class="text-center mt-4">
            <a href="../browse" class="no-default-link">
                <button class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-create-blue hover:bg-create-blue/70 text-white rounded-md pixel-corners">Browse</button>
            </a>
            <a href="../search" class="no-default-link">
                <button class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-create-blue hover:bg-create-blue/70 text-white rounded-md pixel-corners">Search</button>
            </a>
            <a href="../upload" class="no-default-link">
                <button class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-create-blue hover:bg-create-blue/70 text-white rounded-md pixel-corners">Upload</button>
            </a>
        </div>
    </div>
</main>

<style>
@keyframes -global-scale-in {
0% {
    -webkit-transform: translateY(60px);
            transform: translateY(60px);
    opacity: 0;
  }
  100% {
    -webkit-transform: translateY(0);
            transform: translateY(0);
    opacity: 1;
  }
}
</style>