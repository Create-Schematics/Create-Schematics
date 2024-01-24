<script>
  import Arm from "$lib/components/assets/Arm.svelte";
  import Banner from "$lib/components/assets/Banner.svelte";
  import Table from "$lib/components/assets/Table.svelte";
  import InView from "$lib/components/layout/InView.svelte";

  import { toggleMode } from "mode-watcher";

  import { fly } from "svelte/transition";
</script>

<svelte:head>
  <title>Create Schematics</title>
</svelte:head>

<main class="flex flex-col">
  <button on:click={toggleMode}>Toggle Mode</button>
  <div class="bg-blue md:h-[35em]">
    <div class="relative z-0 h-full w-full">
      <div
        class="absolute w-16 h-full bg-slate-100 opacity-30 top-0 left-[20%] -z-10"
      />
      <div
        class="absolute h-16 w-full bg-slate-100 opacity-30 left-0 top-[4rem] -z-10"
      />
      <div
        class="absolute hidden md:block max-h-[35em] max-w-7xl w-full bottom-0 left-1/2 transform -translate-x-1/2 -z-10"
      >
        <Banner />
      </div>
      <div
        class="w-full h-full left-0 md:top-1/4 text-center p-12 flex flex-col justify-center"
      >
        <h1 class="text-5xl md:text-6xl font-bold text-white title-shadow">
          Create Schematics
        </h1>
        <p class="mt-4 text-2xl text-white subtitle-shadow">
          The best way to search, share and download schematics.
        </p>
      </div>
    </div>
  </div>

  <div id="schematics" class="p-12 md:p-24 text-center">
    <InView>
      <div
        class="flex flex-col md:flex-row justify-around gap-12 max-w-6xl m-auto"
        in:fly={{ duration: 700, delay: 100, y: 60 }}
      >
        <div class="h-[20em] md:m-auto flex aspect-square">
          <Table />
        </div>
        <div class="flex flex-col justify-center items-center m-auto">
          <h2 class="text-4xl font-bold">Thousands&nbsp;of Schematics</h2>
          <p class="mt-2 text-2xl">
            All&nbsp;within&nbsp;arms&nbsp;reach, all&nbsp;for&nbsp;free.&nbsp;
          </p>
        </div>
      </div>
    </InView>
  </div>

  <div class="p-12 md:p-24 bg-background-dimmed text-center">
    <InView>
      <div
        class="flex flex-col-reverse md:flex-row justify-around gap-12 max-w-6xl m-auto"
        in:fly={{ duration: 700, delay: 300, y: 60 }}
      >
        <div class="flex flex-col justify-center items-center m-auto">
          <h2 class="text-4xl font-bold">
            Modern, Stable<br />and Easy to use
          </h2>
          <p class="mt-2 text-2xl">Open source and libre.</p>
        </div>
        <div class="flex h-[25em] justify-end p-2 md:m-auto aspect-square">
          <Arm />
        </div>
      </div>
    </InView>
  </div>
  <div
    class="md:w-[calc(100vw-2rem)] max-w-6xl md:px-8 py-8 mt-10 mb-32 mx-auto bg-checker pixel-corners"
  >
    <h2 class="text-4xl font-bold w-full text-center px-8">
      What&nbsp;are&nbsp;you waiting&nbsp;for?
    </h2>
    <div class="text-center mt-4">
      <a href="../browse" class="no-default-link">
        <button
          class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-blue hover:bg-blue/70 text-white rounded-md pixel-corners"
          >Browse</button
        >
      </a>
      <a href="../search" class="no-default-link">
        <button
          class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-blue hover:bg-blue/70 text-white rounded-md pixel-corners"
          >Search</button
        >
      </a>
      <a href="../upload" class="no-default-link">
        <button
          class="px-3 md:px-5 my-2 py-2 mx-1 text-xl font-bold bg-blue hover:bg-blue/70 text-white rounded-md pixel-corners"
          >Upload</button
        >
      </a>
    </div>
  </div>
</main>

<style>
  .title-shadow {
    text-shadow: 4px 4px var(--gray);
  }

  .subtitle-shadow {
    text-shadow: 2px 2px var(--gray);
  }
</style>
