<script lang="ts">
  export let images: string[] = [];
  let x = 0;
  let currentImage = Math.round(x/images.length);
  let e: HTMLDivElement;
  const updateScroll = () => {
    x = e.scrollLeft
    currentImage = Math.round(x/e.clientWidth);
  }
  const scrollToImage = (i: number) => {
    e.scrollTo({left: i*e.clientWidth, behavior: "smooth"})
  }
</script>

<div class="flex relative pixel-corners aspect-video overflow-hidden">
  <div class="overflow-x-scroll scroll-smooth snap-x snap-mandatory flex w-full" bind:this={e} on:scroll={updateScroll} >
    {#each images as image}
      <img src={image} alt="example" class="w-full snap-center" />
    {/each}
  </div>
  <div
    class="bg-black text-white absolute bottom-2 px-2 left-1/2 -translate-x-1/2 pixel-corners flex gap-1 text-opacity-50"
  >
    {#each images as _, i}
      <input type="button" class={`outline-none ${i==currentImage?"text-white":""}`} on:click={e=>scrollToImage(i)} value="■"/>
    {/each}
  </div>
</div>
