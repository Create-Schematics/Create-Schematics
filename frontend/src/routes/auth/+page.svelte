<script lang="ts">
    import GoogleIcon from "$lib/icons/google.svelte"
    import MicrosoftIcon from "$lib/icons/microsoft.svelte";
    import GithubIcon from "$lib/icons/github.svelte"
    import DiscordIcon from "$lib/icons/discord.svelte";
</script>

<svelte:head>
    <title>Register - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-2 pb-28">
    <div class="items-center justify-center bg-checker pixel-corners py-16 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h2 class="text-xl font-bold text-center">Authenticate with</h2>
            <div class="grid grid-cols-2 gap-4 px-3 py-4">
                <button class="dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"
                    on:click={() => { window.location.href = ""}}
                >
                    <GoogleIcon/> Google
                </button>
                <button class="dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"
                    on:click={() => { window.location.href = ""}}
                >
                    <MicrosoftIcon/> Microsoft
                </button>
                <button class="fill-black dark:fill-white bg-white/50 hover:bg-white/30 dark:bg-create-blue/50 dark:hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"
                    on:click={() => { window.location.href = ""}}
                >
                    <GithubIcon /> GitHub
                </button>
                <button class="fill-[#5865F2] dark:fill-white bg-white/50 hover:bg-white/30 dark:bg-create-blue/50 dark:hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"
                    on:click={() => { window.location.href = ""}}
                >
                    <DiscordIcon/> Discord
                </button>
            </div>
            <p class="text-wrap w-72 md:w-96 px-4 text-sm">By creating an account, you agree to Create Schematics' <a href="../terms">Terms of Use</a> and <a href="../privacy">Privacy Policy</a>.</p>
        </div>  
    </div>
</main>