
<script lang="ts">
	import { onMount } from 'svelte';
	import { createScene } from '../lib/homeScene';

	import LINKS from '../data/links';
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
	<hr class="lineDrawToRight"/>
</div>

<div class='links'>
	{#each LINKS as link}
		<a href={link.link}>
			<img style:width = "30px" style:margin-bottom=20px src={link.image} alt='{link.name}'/>
		</a>
	{/each}
</div>

<canvas bind:this={el} />

<style lang="scss">
	.overlay-element {
		margin: 20px;
		position: absolute;
	}

	.links {
		@extend .overlay-element;
		right: 0;
		text-align: right;
		display: flex;
		flex-direction: column;
	}

	.title {
		@extend .overlay-element;
		text-align: left;
		font-size: 3.5em;
	}

	.title > hr {
		margin: 0;
		margin-top: 10px;
		width: 80%;
		/*https://easings.net/#easeInOutQuad*/
		animation: -line-draw-to-right 3s cubic-bezier(0.45, 0, 0.55, 1);
	}

	@keyframes -line-draw-to-right {
		0% {
			width: 0;
		}
		100% {
			width: 80%;
		}
	}

</style>