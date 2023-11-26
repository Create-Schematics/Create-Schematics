<script>
  import "../app.css";
</script>

<body
  class="bg-gray-800 text-white h-screen absolute w-full flex flex-col gap-5 overflow-x-hidden font-pixel checker-bg"
>
  <header
    class="bg-minecraft-ui-dark max-w-6xl p-4 mx-auto mt-3
    w-[calc(100vw-2rem)] flex justify-between items-center pixel-corners"
  >
    <div class="flex items-center gap-3">
      <h1 class="text-xl font-bold">CS</h1>

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
  <slot />
</body>
