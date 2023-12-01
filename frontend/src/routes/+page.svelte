<svelte:head>
    <title>Create Schematics</title>
</svelte:head>

<main>
    <div class="w-[calc(100vw-2rem)] max-w-6xl p-4 mx-auto my-16 md:my-32
    text-center">
        <h1 class="text-6xl font-bold">Create Schematics</h1>
        <p class="mt-4 text-xl">The best way to search, share and download schematics.</p>
        <div class="my-10">
            <a href="../browse" class="no-default-link"><button class="bg-checker px-4 py-2 text-xl font-bold bg-create-blue hover:bg-create-blue/70 text-white rounded-md pixel-corners">Browse Schematics</button></a>
        </div>
        <div class="text-5xl pt-16">
            <a class="no-default-link" href="#schematics">˅</a>
        </div>
    </div>
    <div id="schematics" class="w-[calc(100vw-2rem)] max-w-6xl px-8 py-10 mx-auto bg-minecraft-ui-light dark:bg-minecraft-ui-dark my-16 pixel-corners">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-center md:text-left">
            <div class="flex flex-col justify-center items-center pixel-corners">
                <h2 class="text-4xl font-bold">Thousands&nbsp;of Schematics</h2>
                <p class="mt-2 text-2xl">All&nbsp;within&nbsp;arms&nbsp;reach, all&nbsp;for&nbsp;free.&nbsp;</p>
            </div>
            <div class="text-right ml-auto flex justify-end p-2 bg-checker pixel-corners">
                <img src="https://picsum.photos/700/400">
            </div>
        </div>
    </div>

    <div class="w-[calc(100vw-2rem)] max-w-6xl px-8 py-10 mx-auto bg-minecraft-ui-light dark:bg-minecraft-ui-dark my-16 pixel-corners">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-center">
            <div class="text-left ml-auto flex justify-end p-2 bg-checker pixel-corners">
                <img src="https://picsum.photos/700/401">
            </div>
            <div class="flex flex-col justify-center items-center">
                <h2 class="text-4xl font-bold">Modern,&nbsp;stable &&nbsp;easy&#8209;to&#8209;use</h2>
                <p class="mt-2 text-2xl">Open source and libre.</p>
            </div>
        </div>
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