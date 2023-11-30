<script lang="ts">
    export let form

</script>

<svelte:head>
    <title>Reset Password - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-16 pb-28">
    <div class="items-center justify-center bg-checker pixel-corners py-14 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h1 class="text-xl font-bold text-center p-3">Reset Password</h1>
            <form method="post" action="?/reset" class="">
                <div class="form-item p-2">
                    <!-- <label for="email">Email<sup><small>*</small></sup></label><br> -->
                    <input placeholder="Email" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { 
                            // check email format
                        }}
                        value={form?.email?? ''} id="email" type="email" name="email" required
                    />
                </div>
                <div class="form-item mt-3 flex justify-center">
                    <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-48 md:w-72 h-10 outline-none mx-1 accent-create-blue pixel-corners text-xl">Send Recovery Email</button>
                </div>   
            </form>
        </div>  
    </div>
</main>