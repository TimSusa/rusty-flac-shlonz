<script>
	import Counter from './Counter.svelte'
	import Greet from '../lib/Greet.svelte'
	import Slider from '$lib/Slider.svelte'
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="DEMO for TeamIT" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<img
					src={'https://teamit.fi/wp-content/uploads/2023/09/teamit-spotlight-L-new.svg'}
					alt="Welcome"
				/>
			</picture>
		</span>
		Send IPC to Tauri´s Backend in Rust Universe
	</h1>
	<Slider />
	<Greet />
	<Counter />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
