<script lang="ts">
	import { onMount } from 'svelte';
	import { createScene, panToTable } from '../lib/homeScene';

	import LINKS from '../data/links';
    import Home from '../components/Home.svelte';
	let isZoomed: boolean = false;
	let el: any;

	onMount(() => {
		createScene(el);
	});

	async function panCamera() {
		await panToTable();
		isZoomed = !isZoomed;
	}
</script>

<svelte:head>
	<title>Create Schematics</title>
	<meta name="description" content="Three.js example app built with Svelte" />
</svelte:head>

<div class='title'>
	<div>
		<a>Create: <br/>Schematics</a>
	</div>
	<hr class="lineDrawToRight"/>
	<br/>
</div>

<div class='links'>
	{#each LINKS as link}
		<a href={link.link}>
			<img style:width = "30px" style:margin-bottom=20px src={link.image} alt='{link.name}'/>
		</a>
	{/each}
	<button on:click={panCamera}>Test</button>
</div>

{#if isZoomed}
	<div class='active-page'> 
		
	</div>
{/if}

<canvas bind:this={el} />

<style lang="scss">
	.active-page {
		position: absolute;
		margin: auto;
		width: 50%;
		height: 100%;
		top: 0;
		left: 0;
		right: 0;
		background-color: var(--primary);
		overflow: hidden;
		animation: slide-in-from-top 2s;
	}

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
	}

	.title > div > a {
		box-sizing: border-box;
		font-size: 3.5em;
		color: var(--primary);
		-webkit-text-stroke: 0.1vw var(--primary);

		&::before {
			box-sizing: border-box;
			content: 'Create: Schematics';
			position: absolute;
			left: 0;
			top: 0;
			width: 100%;
			height: 100%;
			color: var(--white);
			overflow: hidden;
			-webkit-text-stroke: 0vw var(--gray);
			border-right: 2px solid #00000000;
			animation: slide-text 3s cubic-bezier(0.45, 0, 0.55, 1);
		}
	}

	.title > hr {
		margin: 0;
		margin-top: 10px;
		width: 100%;
		/*https://easings.net/#easeInOutQuad*/
		animation: -line-draw-to-right 3s cubic-bezier(0.45, 0, 0.55, 1);
	}

	@keyframes slide-in-from-top {
		from {
			height: 0;
			box-shadow: 0 250px var(--primary-accent);
		}
		to {
			height: 100%; 
			box-shadow: 0 0 var(--primary-accent);
		}
	}

	@keyframes slide-text {
		0% {
			width: 0;
			border-right-color: var(--primary-accent);
		}
		90% {
			border-right-color: var(--primary-accent);
		}
		100% {
			width: 100%;
			border-right-color: #00000000;
		}
	}

	@keyframes -line-draw-to-right {
		0% {
			width: 0;
		}
		100% {
			width: 100%;
		}
	}

	@keyframes fade-in-left {
		from {
			left: 0;
		}
		to {
			left: 100%;
		}
	}

</style>