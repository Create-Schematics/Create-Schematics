<script>
  import { page } from "$app/stores";
</script>

<svelte:head>
  <title>{$page.error?.message} - Create Schematics</title>
</svelte:head>
<main
  class="flex items-center justify-center"
  style="height: 90vh; padding-bottom: 15vh;"
>
  <div
    class="text-center text-blue/800 bg-checker pixel-corners pt-20 pb-12 w-[calc(100vw-2rem)]"
    style="text-shadow: 1px 1px 2px #101015, 0 0 1em #B3BDD8, 0 0 0.2em #B3BDD8;"
  >
    <h2 style="font-size: 4rem; line-height: 1;">ERROR</h2>
    <h1 style="font-size: 10rem; line-height: 1;">{$page.status}</h1>
    {#if $page.error}
      <p class="text-4xl">{$page.error.message}</p>
    {/if}
    <a class="no-default-link" href="../">
      <button
        class="bg-blue hover:bg-blue/80 cursor-pointer pixel-corners text-4xl px-5 py-2 mt-10"
        >Home</button
      >
    </a>
  </div>
</main>
