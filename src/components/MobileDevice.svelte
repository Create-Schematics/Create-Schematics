

<div class="device">
	<div class="upload-page">
		<div id="header"> <h1>Create Schematics</h1> </div>
	</div>
</div>

<style lang="scss">
	$device-width: 264px * 1.5;
	$device-height: 544px * 1.5;

	.upload-page {
		display: flex;
		flex-direction: column;

		height: 100%;
		background-color: #63492b;

		> #header {
			background-color: var(--primary);
			height: 10%;
			box-shadow: px 10px var(--primary-accent);
		}
	}

	.device {
		color: var(--white);
		font-family: 'Minecraftia';
		margin: auto;
		right: 2rem;
		height: $device-height;
		width: $device-width;
		padding: 90px 10px;
		border: 5px solid #2f2f2f;
		border-radius: 60px;
		background-color: #171717;
		box-shadow: 0 0 50px 10px rgba(0,0,0,0.4);
		
		&:before, &:after {
			content: '';
			position: absolute;
			z-index: 2;
		}
	}
</style>