<script lang="ts">
    let username: string
    let tagline: string
    let links: {name: string, url: string}[] = [
        {
            name: "",
            url: ""
        },
        {
            name: "",
            url: ""
        }
    ]

    function saveChanges() {
    }
</script>

<svelte:head>
    <title>Account - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center max-w-6xl mx-auto w-[calc(100vw-2rem)] pt-2 pb-28">
    <div class="bg-minecraft-ui-light w-full dark:bg-minecraft-ui-dark pixel-corners p-3">
        <h1 class="text-2xl font-bold mb-4">Edit Account Details</h1>
          <div class="mb-4">
            <label for="username">
                Username
            </label>
            <input type="text" id="username" placeholder="Username (Max 16 Chars)" maxlength="16" bind:value={username} 
            class="border-none pixel-corners p-2 max-w-xl w-full block mb-2" />
            <label for="tagline">
                Profile Tagline
            </label>
            <textarea name="tagline" maxlength="64" id="tagline" rows="2" placeholder="Tell us something about yourself..." bind:value={tagline} 
            class="border-none pixel-corners p-2 max-w-xl w-full block mb-2" />
        </div>    
        <h2>Social Links</h2>
        <div class="max-w-xl w-full flex gap-2">
            <input type="text" id="link0_title" placeholder="Name" maxlength="12" bind:value={links[0].name}
            class="border-none pixel-corners p-2 min-w-[90px] w-[30%] mb-2">
            <input type="text" id="link0_url" placeholder="URL" maxlength="256" bind:value={links[0].url}
            class="border-none pixel-corners p-2 flex-grow min-w-[90px] mb-2" >
        </div>
        <div class="max-w-xl w-full flex gap-2">
            <input type="text" id="link0_title" placeholder="Name" maxlength="12" bind:value={links[1].name}
            class="border-none pixel-corners p-2 min-w-[90px] w-[30%] mb-2">
            <input type="text" id="link0_url" placeholder="URL" maxlength="256" bind:value={links[1].url}
            class="border-none pixel-corners p-2 flex-grow min-w-[90px] mb-2" >
        </div>

          <button type="button" class="bg-blue text-white py-2 px-4 pixel-corners" on:click={saveChanges}>Save</button>
      </div>    
</main>