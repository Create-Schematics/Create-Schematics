<script>
	import Header from '../components/Header.svelte';
    import { getAppAuth, signIn } from '../Auth';
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import authStore from '../stores/authStore';

	onMount(() => {
		if (browser) {
			getAppAuth().onAuthStateChanged((user) => {
				authStore.set({
					isLoggedIn: user !== null,
					user: user ?? undefined,
					firebaseControlled: true
				});
			});
		}
	});

</script>

<main>
	<slot/>
</main>

<style>
    @import '../globals.css';

	:global(body) {
		font-family: 'Minecraftia';
		margin: 0;
		padding: 0;
		overflow: hidden;
	}

	:root {
		background-image: url('/spruce-planks.png');
		background-position: center top;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        overflow-x: hidden;
        background-size: 40%;
        -ms-interpolation-mode: nearest-neighbor;
        /* Firefox */
        image-rendering: crisp-edges;
        /* Chromium + Safari */
        image-rendering: pixelated;
        height: auto;
        color: aliceblue;
	}



</style>