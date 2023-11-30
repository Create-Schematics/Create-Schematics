<script lang="ts">
    export let form

    var usernameTakenError = false;
    var emailFormatError = false;
    var emailMismatchError = false;
    var pwMismatchError = false;
    var pwLengthError = false;

    function updateForm() {
        const username = document.getElementById('username')?.value;
        const email = document.getElementById('email')?.value;
        const confirmEmail = document.getElementById('confirmEmail')?.value;
        const password = document.getElementById('password')?.value;
        const confirmPassword = document.getElementById('confirmPassword')?.value;

        emailFormatError = (/^[\w-\.]+@([\w-]+\.)+[\w-]{2,72}$/.test(email)) ? false : true;
        emailMismatchError = (email == confirmEmail) ? false : true;

        pwLengthError = (password.length > 8) ? false : true; 
        pwMismatchError = (password == confirmPassword) ? false : true;
    }
</script>
<main class="flex items-center justify-center w-screen pb-20">
    <div class="items-center justify-center bg-checker pixel-corners pt-6 pb-6 w-[calc(100vw-2rem)] max-w-6xl">
        <div class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <h1 class="text-2xl font-bold text-center">Register With</h1>
            <div class="grid grid-cols-2 gap-4 px-3 py-4">
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl">Google</button>
                <button class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl">Microsoft</button>
            </div>
            <hr class="my-3 mx-3 opacity-90">
            <h1 class="text-2xl font-bold text-center p-2">Or do it yourself</h1>
            <form method="post" action="?/signup" class="">
                <div class="form-item p-2">
                    <!-- <label for="username">Username<sup><small>*</small></sup></label><br> -->
                    <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
                        value={form?.username?? ''} 
                        on:blur={() => { updateForm() }}
                        id="username" type="text" name="username" required
                    />

                    {#if usernameTakenError === true && document.getElementById('username')?.value.length > 0} 
                        <p class="text-red-500">That username is already taken</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <!-- <label for="email">Email<sup><small>*</small></sup></label><br> -->
                    <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { updateForm() }}
                        value={form?.email?? ''} id="email" type="email" name="email" required
                    />
                    {#if emailFormatError === true && document.getElementById('email')?.value.length > 0} 
                        <p class="text-red-500">Must be a valid email address</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <!-- <label for="confirmEmail">Confirm Email<sup><small>*</small></sup></label><br> -->
                    <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
                        on:blur={() => { updateForm() }}
                        value={form?.email?? ''} id="confirmEmail" type="email" name="confirmEmail" required
                    />
                    {#if emailMismatchError === true && document.getElementById('confirmEmail')?.value.length > 0} 
                        <p class="text-red-500">Email addressses much match</p>
                    {/if}
                </div>

                <div class="form-item p-2">
                    <!-- <label for="password">Password<sup><small>*</small></sup></label><br> -->
                    <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { updateForm() }}
                        id="password" type="password" name="password" required
                    />
                    {#if pwLengthError === true && document.getElementById('password')?.value.length > 0} 
                        <p class="text-red-500">Password must be at least 8 characters long</p>
                    {/if}
                </div>
                <div class="form-item p-2">
                    <!-- <label for="password">Confirm Password<sup><small>*</small></sup></label><br> -->
                    <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                        on:blur={() => { updateForm() }}
                        id="confirmPassword" type="password" name="confirmPassword" required
                    />
                    {#if pwMismatchError === true && document.getElementById('confirmPassword')?.value.length > 0} 
                        <p class="text-red-500">Passwords much match</p>
                    {/if}
                </div>

                <div class="form-item mt-3">
                    <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners text-xl">Register</button>
                </div>   
            </form>
        </div>  
    </div>
</main>