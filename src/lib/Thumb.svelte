<script lang="ts">
	import { createEventDispatcher } from 'svelte'
	import handle from './slider.js'
	const dispatch = createEventDispatcher()
	let pos = 0
	let active = false
	export { pos }
</script>

<div
	tabindex="0"
	role="slider"
	aria-valuenow={pos}
	class="thumb"
	style={`left: ${pos * 100}%;`}
	use:handle
	on:dragstart={() => ((active = true), dispatch('active', true))}
	on:drag={({ detail: v }) => (pos = v)}
	on:dragend={() => ((active = false), dispatch('active', false))}
>
	<div class="thumb-content" class:active>
		
		<slot />
	</div>
	<div>
		{pos.toString().substr(0, 4) }

	</div>
</div>

<style>
	.thumb {
		position: absolute;
		top: 50%;
		width: 0;
		height: 0;
	}

	.thumb-content {
		position: relative;
		width: fit-content;
		height: fit-content;
		transform: translate(-50%, -50%);
	}

	.thumb-content::before {
		content: '';
		position: absolute;
		width: 200%;
		height: 200%;
		transform: translate(-25%, -25%) scale(0);
		border-radius: 100vh;
		background: var(--thumb-bg, #5784fd);
		opacity: 30%;
		transition: transform 100ms ease-in-out;
	}

	.thumb-content.active::before {
		transform: translate(-25%, -25%) scale(1);
	}
</style>
