<script lang="ts">
    import GoogleIcon from "$lib/icons/google.svelte"
    import MicrosoftIcon from "$lib/icons/microsoft.svelte";

    /* let usernameTakenError = false;
    let emailFormatError = false;
    let pwMismatchError = false;
    let pwLengthError = false;
    let username:string
    let email:string
    let password:string
    let confirmPassword:string
    let errorOccured = false;

    function updateErrors() {
        if (email?.length > 0) {
            emailFormatError = (/^[\w-\.]+@([\w-]+\.)+[\w-]{2,72}$/.test(email)) ? false : true;
        } else { emailFormatError = false }
        if (password?.length > 0) {
            pwLengthError = (password.length > 8) ? false : true; 
        } else { pwLengthError = false }
        if (password?.length > 0 && confirmPassword?.length > 0) {
            pwMismatchError = (password == confirmPassword) ? false : true;
        } else { pwMismatchError = false }
        return (emailFormatError === false && pwLengthError === false && pwMismatchError === false && usernameTakenError === false)
    }

    function handleSubmit(event: Event) {
        if (updateErrors() == true) { // No Errors Found
            console.log("Submitting form");
            console.log([email, password, confirmPassword, username]);
        } else { // Errors Found
            console.log("Errors in form");
            errorOccured = true;
        }
    } */
</script>

<svelte:head>
    <title>Register - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-2 pb-28">
    <div class="items-center justify-center bg-checker pixel-corners py-6 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h2 class="text-xl font-bold text-center">Register with</h2>
            <div class="grid grid-cols-2 gap-4 px-3 py-4">
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"><GoogleIcon/> Google</button>
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"><MicrosoftIcon/> Microsoft</button>
            </div>
            <!--<hr class="my-3 border-slate-800 mx-3">
            <h2 class="text-xl font-bold text-center p-3">Or do it manually</h2>
            <form method="post" action="?/signup" on:submit|preventDefault={handleSubmit}>
                <div class="form-item p-2">
                    <input placeholder="Email" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        bind:value={email} on:blur={() => { updateErrors() }}
                        id="email" type="email" name="email" required
                    />
                    {#if emailFormatError === true} 
                        <p class="text-red-500">Must be a valid email address</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <input placeholder="Username" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
                        bind:value={username} on:blur={() => { updateErrors() }}
                        id="username" type="text" name="username" required
                    />

                    {#if usernameTakenError === true} 
                        <p class="text-red-500">That username is already taken</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <input placeholder="Password" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                    bind:value={password} on:blur={() => { updateErrors() }}
                    id="password" type="password" name="password" required
                    />
                    {#if pwLengthError === true} 
                        <p class="text-red-500">Password must be >8 characters long</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <input placeholder="Confirm password" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                    bind:value={confirmPassword} on:blur={() => { updateErrors() }}
                    id="confirmPassword" type="password" name="confirmPassword" required
                    />
                    {#if pwMismatchError === true} 
                        <p class="text-red-500">Passwords much match</p>
                    {/if}
                </div>-->

                <!--{#if errorOccured === true} 
                <p class="text-red-500 text-wrap w-72 px-2 md:w-96 text-m">An error occured while trying to submit the form. Please try again.</p>
                {/if}
                <div class="form-item mt-5 flex justify-center">
                    <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-64 md:w-72 h-10 outline-none mx-1 accent-create-blue pixel-corners text-xl">Register</button>
                </div>
            </form>-->  
            <p class="text-wrap w-72 md:w-96 px-4 pt-4 text-sm">By creating an account, you agree to Create Schematics' <a href="../terms">Terms of Use</a> and <a href="../privacy">Privacy Policy</a>.</p>
            <div class="text-l font-bold text-center mt-2">
                <h2><a href="../auth/sign-in">Sign in to existing account ➜</a></h2>
            </div>
        </div>  
    </div>
</main>