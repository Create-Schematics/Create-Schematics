<script lang="ts">
  import Header from "$lib/components/layout/Header.svelte";
  import Footer from "$lib/components/layout/Footer.svelte";

  import type { PageData } from "./$types";
  import "../app.css";

  import { ModeWatcher } from "mode-watcher";

  export let data: PageData;
  const user = data.currentUser;
</script>

<ModeWatcher />

<body
  class=" bg-create-blue/40 dark:bg-gray-800 dark:text-white absolute w-full flex flex-col gap-3 font-pixel min-h-screen"
>
  <Header {user} />

  <main class="max-w-6xl m-auto p-4 min-h-screen">
    <slot />
  </main>

  <Footer />
</body>
