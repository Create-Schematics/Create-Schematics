<script lang="ts">
    // TODO; Implement Firebase Auth On Load
</script>


<div class='login-wrapper'>
    <form class="login-form">
        <h1>Login</h1>
        <hr/>

        <input
            type="text"
            placeholder="Email..."
        />

        <input
            type="text"
            placeholder="Password..."
        />
        
        <button>Submit</button>

    </form>
</div>

<style lang="scss">
    .login-form {
        padding: 10px;
        margin: auto;
        width: 95%;
        border-style: solid;
        border-color: var(--white);
        border-radius: 20px;
        height: auto;
        gap: 20px;
        display: flex;
        flex-direction: column;

        > input[type="text"] {
            color: aliceblue;
            font-family: 'Minecraftia';
            background-color: var(--primary);
            border-style: solid;
            border-color: var(--white);
            border-radius: 10px;
            padding: 10px;
        }

        > input[type="text"]::placeholder {
            color: var(--grey);
        }
    }
</style>