<script lang="ts">
  export let images: string[] = [];
  let currentImage = 0;
  let e: HTMLDivElement;
</script>

<div
  class="flex relative pixel-corners aspect-video overflow-hidden bg-blue/30 bg-checker w-full"
>
  <div
    class="overflow-x-scroll scroll-smooth snap-x snap-mandatory flex w-full"
    bind:this={e}
  >
    <img
      src={images[currentImage]}
      alt="example"
      class="h-full w-full snap-center object-cover"
    />
  </div>
  <div
    class="bg-black text-white absolute bottom-2 px-2 left-1/2 -translate-x-1/2 pixel-corners flex gap-1 text-opacity-50"
  >
    {#each images as _, i}
      <input
        type="button"
        class={`outline-none ${i == currentImage ? "text-white" : ""}`}
        on:click={() => (currentImage = i)}
        value="■"
      />
    {/each}
  </div>
</div>
