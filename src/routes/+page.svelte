
<script lang="ts">
	import { onMount } from 'svelte';
	import { createScene } from '../lib/homeScene';
	let el: any;
	onMount(() => {
		createScene(el);
	});
</script>

<svelte:head>
	<title>Create Schematics</title>
	<meta name="description" content="Three.js example app built with Svelte" />
</svelte:head>

<div class='title'>
	Create: <br/>Schematics
	<hr/>
</div>

<div class='links'>
	
</div>

<canvas bind:this={el} />

<style lang="scss">
	.title {
		margin: 20px;
		font-size: 3.5em;
		position: absolute;

		> hr {
			margin: 0;
			margin-top: 10px;
			width: 80%;
		}
	}
</style>