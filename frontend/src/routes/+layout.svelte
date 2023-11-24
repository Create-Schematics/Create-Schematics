<script>
  import "../app.css";
</script>

<body
  class="bg-gray-950 text-white h-screen absolute w-full flex flex-col gap-5 overflow-x-hidden"
>
  <header
    class="bg-create-blue/5 max-w-6xl p-4 mx-auto mt-3
    rounded-2xl w-[calc(100vw-2rem)] flex justify-between items-center
    ring-1 ring-create-blue/20"
  >
    <div class="flex items-center gap-3">
      <h1 class="text-xl font-bold">CS</h1>

      <input
        type="search"
        placeholder="search schematics..."
        class="accent-create-blue h-10 outline-none px-3 bg-black/30 rounded-xl ring-1 ring-create-blue/10"
      />
    </div>
    <div class="flex items-center gap-3">
      <div
        class="rounded-xl bg-create-blue/50 ring-create-blue/70
        ring-1 flex h-10 w-10 text-center hover:bg-create-blue/80 cursor-pointer
        items-center justify-center text-3xl font-black font-mono"
      >
        +
      </div>
      <div
        class="w-10 h-10 bg-white/50 rounded-xl ring-create-blue/50 ring-1 overflow-hidden"
      >
        <img src="https://picsum.photos/50" alt="" />
      </div>
    </div>
  </header>
  <slot />
</body>
