<script>
  import "../app.css";
</script>

<body
  class="bg-gray-800 text-white h-screen absolute w-full flex flex-col gap-3 overflow-x-hidden font-pixel bg-checker"
>
  <header
    class="bg-minecraft-ui-dark max-w-6xl p-4 mx-auto mt-3
    w-[calc(100vw-2rem)] flex justify-between items-center pixel-corners"
  >
    <div class="flex items-center gap-3">
      <a href="/"><h1 class="text-xl font-bold">CS</h1></a>

      <input
        type="search"
        placeholder="search schematics..."
        class="accent-create-blue h-10 outline-none px-3 bg-black/30 pixel-corners"
      />
    </div>
    <div class="flex items-center gap-3">
      <div
        class="l bg-create-blue/50
        flex h-10 w-10 text-center hover:bg-create-blue/80 cursor-pointer
        items-center justify-center text-3xl font-black pixel-corners"
      >
        +
      </div>
      <div
        class="w-10 h-10 bg-white/50 overflow-hidden pixel-corners"
      >
        <img src="https://picsum.photos/50" alt="" />
      </div>
    </div>
  </header>
  <main class="flex-grow">
    <slot />
  </main>
  <footer class="bg-minecraft-ui-dark opacity-75 p-4 justify-between items-center">

    <div class="container mx-auto text-center md:text-left p-4 opacity-100 ">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-w-6xl mx-auto">

        <div class="mb-4 md:mb-0 items-center">
          <div class="w-fit mx-auto">
            <h2 class="text-lg font-semibold mb-2">Create Schematics</h2>
            <p class="text-m">
              Create Schematics is <a href="https://github.com/Create-Schematics/Create-Schematics">open source</a>.<br>
              <a href="https://discord.gg/GJsQadv9Mc">Discord</a><br>
              <a href="mailto:contact@createschematics.com">Email</a>
            </p>
          </div>
        </div>

        <div class="mb-4 md:mb-0 items-center">
          <div class="w-fit mx-auto">
            <h2 class="text-lg font-semibold mb-2">Resources</h2>
            <p class="text-m">
              <a href="../terms">Terms of Use</a><br>
              <a href="../privacy">Privacy</a><br>
              <a href="../rules">Upload Rules</a>
            </p>
          </div>
        </div>
      </div>

      <div class="mx-auto mt-8 text-sm text-gray-500 text-center opacity-100">
        <p>&copy; 2023. <a href="https://github.com/Create-Schematics/Create-Schematics/blob/master/LICENSE">Licensed under the MIT License</a>.</p>

        <p>NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG.<br>NOT APPROVED BY OR ASSOCIATED WITH THE CREATE MOD.</p>
      </div>
  </footer>
</body>
