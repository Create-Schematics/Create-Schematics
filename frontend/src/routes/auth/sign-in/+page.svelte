<script lang="ts">
    import GoogleIcon from "$lib/icons/google.svelte"
    import MicrosoftIcon from "$lib/icons/microsoft.svelte";
    export let form

    let emailFormatError = false;
    let pwLengthError = false;
    let email:string
    let password:string

    function updateErrors() {
        if (email?.length > 0) {
            emailFormatError = (/^[\w-\.]+@([\w-]+\.)+[\w-]{2,72}$/.test(email)) ? false : true;
        } else { emailFormatError = false }
        if (password?.length > 0) {
            pwLengthError = (password.length > 8) ? false : true; 
        } else { pwLengthError = false }
    }
</script>

<svelte:head>
    <title>Sign in - Create Schematics</title>
</svelte:head>

<main class="flex items-center justify-center w-screen pt-16 pb-28">
    <div class="items-center justify-center bg-checker pixel-corners py-14 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h2 class="text-xl font-bold text-center">Sign in with</h2>
            <div class="grid grid-cols-2 gap-4 px-3 py-4">
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"><GoogleIcon/> Google</button>
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl whitespace-nowrap"><MicrosoftIcon/> Microsoft</button>
            </div>
            <hr class="my-3 border-slate-800 mx-3">
            <h2 class="text-xl font-bold text-center px-3 pb-1 pt-4 ">Or use an email & password</h2>
            <form method="post" action="?/login" class="">
                <div class="form-item p-2">
                    <!-- <label for="email">Email<sup><small>*</small></sup></label><br> -->
                    <input placeholder="Email" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { updateErrors() }}
                        value={form?.email?? ''} id="email" type="email" name="email" required
                    />
                    {#if emailFormatError === true} 
                        <p class="text-red-500">Must be a valid email address</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <!-- <label for="password">Password<sup><small>*</small></sup></label><br> -->
                    <input placeholder="Password" class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { updateErrors() }}
                        id="password" type="password" name="password" required
                    />
                    {#if pwLengthError === true} 
                        <p class="text-red-500">Password must be at least 8 characters long</p>
                    {/if}
                </div>

                <div class="form-item mt-5 flex justify-center">
                    <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-64 md:w-72 h-10 outline-none mx-1 accent-create-blue pixel-corners text-xl">Sign in</button>
                </div>   
            </form>
            <div class="text-l font-bold text-center mt-2">
                <h2><a href="../auth/reset-password">Reset Password</a></h2>
                <h2><a href="../auth/register">Create an Account ➜</a></h2>
            </div>
        </div>  
    </div>
</main>