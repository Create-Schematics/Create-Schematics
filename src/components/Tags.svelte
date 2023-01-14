<script lang="ts">
    export let tags: string[] = [];
</script>

<div class='tag-container'>
    {#each tags as tag}
        <div class='tag'>{tag}</div>
    {/each}
</div>

<style lang="scss">
    .tag-container {
        display: flex;
        flex-direction: row;
        gap: 3px;
        
        > div {
            background-color: var(--primary);
            color: var(--white);
            border-radius: 10px;
            padding: 7px;
            transition: all 0.2s;
        }

        > div:hover {
            border-radius: 20px;
            background-color: var(--primary-accent);
        }
    }
</style>