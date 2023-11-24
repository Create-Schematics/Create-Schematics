<script lang="ts">
  export let images: string[] = [];
</script>

<div class="flex overflow-hidden relative rounded-xl">
  {#each images as image}
    <img src={image} alt="example" class="w-full flex aspect-video" />
  {/each}
  <div
    class="bg-black/50 absolute bottom-1 px-2 rounded-full left-1/2 -translate-x-1/2"
  >
    {#each images as image}
      <span>●</span>
    {/each}
  </div>
</div>
