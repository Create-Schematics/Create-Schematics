<script lang="ts">
    let username: string
    let description: string

    function saveChanges() {
    }
</script>

<svelte:head>
    <title>Account - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-2 pb-28">
    <div class="bg-minecraft-ui-light dark:bg-minecraft-ui-dark pixel-corners p-3">
        <h2 class="text-2xl font-bold mb-4">Account Details</h2>
          <div class="mb-4">
            <input type="text" id="username" placeholder="Username" bind:value={username} class="border-none pixel-corners p-2 w-full" />
            <textarea name="description" id="description" rows="3" placeholder="Description" bind:value={description} class="border-none pixel-corners p-2 w-full" />
        </div>    
          <button type="button" class="bg-blue-500 text-white py-2 px-4 rounded-md" on:click={saveChanges}>Save</button>
      </div>    
</main>