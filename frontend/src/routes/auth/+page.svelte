<script lang="ts">
  import GoogleIcon from "$lib/components/icons/google.svelte";
  import MicrosoftIcon from "$lib/components/icons/microsoft.svelte";
  import GithubIcon from "$lib/components/icons/github.svelte";
  import DiscordIcon from "$lib/components/icons/discord.svelte";
  import { apiBaseUrl } from "$lib/requests";
</script>

<svelte:head>
  <title>Auth - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center pt-2 pb-28">
  <div
    class="items-center justify-center bg-checker pixel-corners py-16 w-[calc(100vw-2rem)] max-w-6xl"
  >
    <div
      class="mx-auto w-fit p-4 pixel-corners bg-background-dimmed flex flex-col items-center"
    >
      <h2 class="text-xl font-bold text-center">Authenticate with</h2>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 px-3 py-4">
        <a
          href={`${apiBaseUrl}/auth/google`}
          class="flex flex-col items-center no-default-link dark:bg-blue bg-white/50 hover:bg-white/30 dark:hover:bg-blue/80 p-3 pixel-corners"
        >
          <GoogleIcon />
          Google
        </a>
        <a
          href={`${apiBaseUrl}/auth/microsoft`}
          class="flex flex-col items-center no-default-link dark:bg-blue bg-white/50 hover:bg-white/30 dark:hover:bg-blue/80 p-3 pixel-corners"
        >
          <MicrosoftIcon />
          Microsoft
        </a>
        <a
          href={`${apiBaseUrl}/auth/github`}
          class="flex flex-col items-center no-default-link dark:bg-blue bg-white/50 hover:bg-white/30 dark:hover:bg-blue/80 p-3 pixel-corners"
        >
          <GithubIcon />
          GitHub
        </a>
        <a
          href={`${apiBaseUrl}/auth/discord`}
          class="flex flex-col items-center no-default-link dark:bg-blue bg-white/50 hover:bg-white/30 dark:hover:bg-blue/80 p-3 pixel-corners"
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
