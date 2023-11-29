<script lang="ts">
    export let form

    var usernameTakenError = false;
    var emailFormatError = false;
    var emailMismatchError = false;
    var pwMismatchError = false;
    var pwLengthError = false;
</script>
<main class="flex items-center justify-center w-screen pb-20">
    <div class="items-center justify-center bg-checker pixel-corners pt-6 pb-6 w-[calc(100vw-2rem)] max-w-6xl">
    <form method="post" action="?/signup" class="mx-auto w-fit p-4 pixel-corners bg-minecraft-ui-light dark:bg-minecraft-ui-dark">
            <div class="form-item p-2">
            <label for="username">Username<sup><small>*</small></sup></label><br>
            <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
                   value={form?.username?? ''} 
                   on:blur={() => {
                        // Check if username taken
                   }}
                   id="username" type="text" name="username" required/>
        </div>
        <div class="form-item p-2">
            <label for="email">Email<sup><small>*</small></sup></label><br>
            <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                   on:blur={() => {
                        if (/^[\w-\.]+@([\w-]+\.)+[\w-]{2,72}$/.test(document.getElementById('email')?.value)) {
                            emailFormatError = false;
                        } else {
                            emailFormatError = true;
                        }
                   }}
                   value={form?.email?? ''} id="email" type="email" name="email" required/>
        </div>
        <div class="form-item p-2">
            <label for="confirmEmail">Confirm Email<sup><small>*</small></sup></label><br>
            <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners"
            on:blur={() => {
                if (document.getElementById('email')?.value == document.getElementById('confirmEmail')?.value) {
                    emailMismatchError = false;
                } else {
                    emailMismatchError = true;
                }
                console.log("Email Mismatch: " + emailMismatchError)
            }}

            value={form?.email?? ''} id="confirmEmail" type="email" name="confirmEmail" required/>
        </div>

        <div class="form-item p-2">
            <label for="password">Password<sup><small>*</small></sup></label><br>
            <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                on:blur={() => {
                    if (document.getElementById('password')?.value.length > 8) {
                        pwLengthError = true;
                    } else {
                        pwLengthError = false;
                    }
                }}
                id="password" type="password" name="password" required
            />
            <!-- This isn't reactive and I don't know how to make it reactive. -->
            {#if pwLengthError === true} 
                 <p class="text-red-500">Password must be at least 8 characters long</p>
            {/if}
        </div>
        <div class="form-item p-2">
            <label for="password">Confirm Password<sup><small>*</small></sup></label><br>
            <input class="accent-create-blue w-72 md:w-96 h-10 outline-none px-3 dark:bg-black/30 pixel-corners" 
                   class:fieldError={form?.passwordMismatch}
                   on:blur={() => {
                   }}
                   id="confirmPassword" type="password" name="confirmPassword" required
            />
        </div>


        <div class="form-item">
            {#if form?.error}
            <small>{form?.message}</small>
            {/if}
            
            {#if form?.passwordsMismatch}
            <p class="text-red-500">Passwords do not match.</p>
            {/if}
        </div>

        <div class="form-item">
            <button type="submit" class="bg-create-blue/50 hover:bg-create-blue/80 w-full h-10 outline-none px-3 accent-create-blue pixel-corners">Register</button>
        </div>   
    </form>
    </div>
</main>