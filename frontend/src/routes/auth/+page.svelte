<script lang="ts">
  import GoogleIcon from "$lib/components/icons/google.svelte";
  import MicrosoftIcon from "$lib/components/icons/microsoft.svelte";
  import GithubIcon from "$lib/components/icons/github.svelte";
  import DiscordIcon from "$lib/components/icons/discord.svelte";
</script>

<svelte:head>
  <title>Auth - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center pt-2 pb-28">
  <div
    class="items-center justify-center bg-checker pixel-corners py-16 w-[calc(100vw-2rem)] max-w-6xl"
  >
    <div
      class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark flex flex-col items-center"
    >
      <h2 class="text-xl font-bold text-center">Authenticate with</h2>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 px-3 py-4">
        <a
          href=""
          class="flex flex-col items-center no-default-link dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 p-3 pixel-corners"
        >
          <GoogleIcon />
          Google
        </a>
        <a
          href=""
          class="flex flex-col items-center no-default-link dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 p-3 pixel-corners items-center"
        >
          <MicrosoftIcon />
          Microsoft
        </a>
        <a
          href=""
          class="flex flex-col items-center no-default-link dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 p-3 pixel-corners items-center"
        >
          <GithubIcon />
          GitHub
        </a>
        <a
          href=""
          class="flex flex-col items-center no-default-link dark:bg-create-blue/50 bg-white/50 hover:bg-white/30 dark:hover:bg-create-blue/80 p-3 pixel-corners items-center"
        >
          <DiscordIcon />
          Discord
        </a>
      </div>
      <p class="text-wrap w-72 md:w-96 px-4 text-sm text-center">
        By creating an account, you agree to Create Schematics' <a
          href="../terms">Terms of Use</a
        >
        and <a href="../privacy">Privacy Policy</a>.
      </p>
    </div>
  </div>
</main>
