<script lang="ts">
  import type { components } from "$lib/openapiSchema";

  export let user: components["schemas"]["User"] | undefined;
</script>

<header class="bg-background-dimmed p-4 w-full sticky top-0 z-10">
  <section class="w-full flex grid-cols-3 md:grid gap-3 md:gap-8">
    <div class="flex items-center gap-4">
      <a href="/">
        <img src="/favicon.png" class="w-10" alt="logo" />
      </a>

      <span class="text-xl font-bold hidden md:block"> Create Schematics </span>
    </div>

    <search title="schematics" class="flex-grow max-w-6xl">
      <form action="browse" autocomplete="off">
        <input
          type="search"
          name="term"
          placeholder="Search Schematics..."
          class="h-10 outline-none w-full px-3 bg-background pixel-corners"
        />
      </form>
    </search>

    <div class="flex justify-end gap-3">
      {#if user}
        <a
          href="/upload"
          class="bg-blue no-default-link flex h-10 w-10 text-center hover:bg-blue/80 cursor-pointer text-white items-center justify-center text-4xl font-mono font-black pixel-corners"
        >
          +
        </a>
        <a
          href={`/user/${user.username}`}
          class="w-10 h-10 bg-white/50 overflow-hidden pixel-corners"
        >
          <img src={user.avatar} alt="avatar" />
        </a>
      {:else}
        <a
          href="/auth"
          class="h-10 bg-blue overflow-hidden pixel-corners flex items-center p-3 no-default-link whitespace-nowrap text-white"
        >
          Sign in
        </a>
      {/if}
    </div>
  </section>
</header>
