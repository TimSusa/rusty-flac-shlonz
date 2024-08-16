<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api';

	let name = ''
	let greetMsg = ''
    let items = [];
    let page = 0;
    const limit = 50; // Number of items per request
    let loading = false;

    async function fetchItems(page) {
        loading = true;
        try {
            const newItems = await invoke('get_items', { page, limit });
            items = [...items, ...newItems];
        } catch (error) {
            console.error("Error fetching items:", error);
        } finally {
            loading = false;
        }
    }

    function loadMore() {
        page += 1;
        fetchItems(page);
    }

    onMount(() => {
        fetchItems(page);
    });
	async function greet() {
		greetMsg = await invoke('greet', { name })
	}
</script>

<div>
	<h1>Items List</h1>
    <ul>
        {#each items as item}
            <li>{item.name}</li>
        {/each}
    </ul>
    {#if loading}
        <p>Loading...</p>
    {/if}
    <button on:click={loadMore} disabled={loading}>
        Load More
    </button>
	<input id="greet-input" placeholder="Send IPC Event to rust backen" bind:value={name} />
	<button on:click={greet}>miiiiiiiiii</button>
	<p>{greetMsg}</p>
</div>


<style>
    main {
        padding: 20px;
    }
    button:disabled {
        background-color: #ccc;
    }
</style>