<script lang="ts">
  import "../app.css";
  import SunIcon from "$lib/icons/sun.svelte";
  import MoonIcon from "$lib/icons/moon.svelte";
  import { browser } from "$app/environment";
  import type { PageData } from "./$types";

  export let data: PageData;
  const { user } = data;

  let darkMode = false;

  function handleSwitchDarkMode() {
    darkMode = !darkMode;
    localStorage.setItem("theme", darkMode ? "dark" : "light");

    document.body.classList.toggle("dark");
  }

  if (browser) {
    const storedTheme = localStorage.getItem("theme");
    console.log(storedTheme);
    if (storedTheme === "dark") {
      document.body.classList.add("dark");
      darkMode = true;
    } else if (storedTheme === "light") {
      document.body.classList.remove("dark");
      darkMode = false;
    } else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      document.body.classList.add("dark");
      darkMode = true;
    }
  }
</script>

<body
  class=" bg-create-blue/40 dark:bg-gray-800 dark:text-white h-screen absolute w-full flex flex-col gap-3 overflow-x-hidden font-pixel"
>
  <header
    class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark max-w-6xl p-4 mx-auto mt-3
    w-[calc(100vw-2rem)] flex justify-between items-center pixel-corners"
  >
    <div class="flex items-center gap-3">
      <a href="/" class=" w-8 md:w-10">
        <img src="/favicon.png" class="w-10" alt="logo" />
      </a>

      <search title="schematics">
        <form action="browse" autocomplete="off">
          <input
            type="search"
            name="q"
            placeholder="search schematics..."
            class="accent-create-blue h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
          />
        </form>
      </search>
    </div>
    <div class="flex items-center gap-3">
      <button
        class="w-12 h-12 outline-none accent-create-blue pixel-corners text-xl whitespace-nowrap"
        on:click={() => {
          handleSwitchDarkMode();
        }}
      >
        <div
          class="fill-minecraft-ui-dark/90 dark:fill-white flex items-center justify-center"
        >
          {#if darkMode}
            <SunIcon />
          {:else}
            <MoonIcon />
          {/if}
        </div>
      </button>
      {#if user.ok}
        <a
          href="/upload"
          class="bg-create-blue/80 no-default-link
        flex h-10 w-10 text-center hover:bg-create-blue/80 cursor-pointer text-white
        items-center justify-center text-4xl font-mono font-black pixel-corners"
        >
          +
        </a>
        <a
          href={`/user/${data.user.data.username}`}
          class="w-10 h-10 bg-white/50 overflow-hidden pixel-corners"
        >
          <!-- TODO: make it display the user avatar image -->
          <!-- <img src={user.data.user_id} alt="avatar" /> -->
        </a>
      {:else}
        <a
          href="/auth"
          class="h-10 bg-create-blue/50 overflow-hidden pixel-corners flex items-center p-3 no-default-link"
        >
          Sign in
        </a>
      {/if}
    </div>
  </header>
  <main class="flex-grow">
    <slot />
  </main>
  <footer class="p-4 justify-between items-center">
    <div class="container mx-auto text-center md:text-left p-4 opacity-100">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-w-xl mx-auto">
        <div class="mb-4 md:mb-0 items-center">
          <div class="w-fit mx-auto">
            <h2 class="text-lg font-semibold mb-2">Create Schematics</h2>
            <p class="text-m">
              We're <a
                href="https://github.com/Create-Schematics/Create-Schematics"
                >open source</a
              ><br />
              <a href="https://discord.gg/GJsQadv9Mc">Discord</a><br />
              <a href="mailto:contact@createschematics.com">Email</a>
            </p>
          </div>
        </div>

        <div class="mb-4 md:mb-0 items-center">
          <div class="w-fit mx-auto">
            <h2 class="text-lg font-semibold mb-2">Resources</h2>
            <p class="text-m">
              <a href="../terms">Terms of Use</a><br />
              <a href="../privacy">Privacy</a><br />
              <a href="../rules">Upload Rules</a>
            </p>
          </div>
        </div>
      </div>

      <div class="mx-auto mt-8 text-sm text-gray-500 text-center opacity-100">
        <p>
          &copy; 2023. <a
            class="text-slate-500 hover:text-slate-400"
            href="https://github.com/Create-Schematics/Create-Schematics/blob/master/LICENSE"
            >Licensed under the MIT License</a
          >.
        </p>

        <p>
          NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH
          MOJANG.<br />NOT APPROVED BY OR ASSOCIATED WITH THE CREATE MOD.
        </p>
      </div>
    </div>
  </footer>
</body>
