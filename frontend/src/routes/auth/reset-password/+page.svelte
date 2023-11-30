<script lang="ts">
    let emailFormatError = false;
    let email:string
    let formSuccess = false;
    let errorOccured = false;

    function updateErrors() {
        if (email?.length > 0) {
            emailFormatError = (/^[\w-\.]+@([\w-]+\.)+[\w-]{2,72}$/.test(email)) ? false : true;
        } else { emailFormatError = false }
        return (emailFormatError === false)
    }

    function handleSubmit(event: Event) {
        if (updateErrors() == true) { // No errors found
            console.log("Submitting form");
            console.log(email);
            formSuccess = true;
        } else { // Errors Found
            console.log("Errors in form");
            formSuccess = false;
        }
    }
</script>

<svelte:head>
    <title>Reset Password - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-16 pb-28">
    <div class="items-center justify-center bg-checker pixel-corners py-14 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h1 class="text-xl font-bold text-center p-3">Reset Password</h1>
            <form method="post" action="?/reset" on:submit|preventDefault={handleSubmit}>
                <div class="form-item p-2">
                    <!-- <label for="email">Email<sup><small>*</small></sup></label><br> -->
                    <input placeholder="Email" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                    on:blur={() => { updateErrors() }}
                    bind:value={email} id="email" type="email" name="email" required
                />
                {#if emailFormatError === true} 
                    <p class="text-red-500 px-1">Must be a valid email address</p>
                {/if}
                {#if formSuccess === true} 
                    <p class="text-green-500 text-wrap w-72 px-1 md:w-96 text-m">Success! If an account with that Email exists, a recovery email has been sent.</p>
                {/if}
                {#if errorOccured === true} 
                    <p class="text-red-500 text-wrap w-72 px-1 md:w-96 text-m">An error occured while trying to submit the form. Please try again.</p>
                {/if}

                </div>
                <div class="form-item mt-3 flex justify-center">
                    <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-48 md:w-72 h-10 outline-none mx-1 accent-create-blue pixel-corners text-xl">Send Recovery Email</button>
                </div>   
            </form>
            <div class="text-l font-bold text-center mt-2">
                <h2><a href="../auth/sign-in">Sign in ➜</a></h2>
            </div>
        </div>  
    </div>
</main>