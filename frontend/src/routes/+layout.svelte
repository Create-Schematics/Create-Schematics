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
  class="bg-background absolute w-full flex flex-col font-pixel min-h-screen content-between"
>
  <Header {user} />

  <div class="flex-grow">
    <slot />
  </div>

  <Footer />
</body>
