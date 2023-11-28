<script lang="ts">
  export let images: string[] = [];
</script>

<div class="flex overflow-hidden relative pixel-corners">
  {#each images as image}
    <img src={image} alt="example" class="w-full flex aspect-video" />
  {/each}
  <div
    class="bg-black text-white absolute bottom-1 px-2 left-1/2 -translate-x-1/2 pixel-corners flex gap-1"
  >
    {#each images as image}
      <span>■</span>
    {/each}
  </div>
</div>
