
<script lang="ts">
	import { onMount } from 'svelte';
	import { Store } from "tauri-plugin-store-api";
    import type { SteamGame } from '$lib/types/Steam';
	
	const store = new Store(".settings.dat");

    export let props;
	export let page;

	onMount(async () => {
		let game = (await (await fetch(`https://store.caiocinel.com/api/game?id=${props.gameID}`)).json()).data[Number(props.gameID)].data;
        if(game)
            store.set("game"+props.gameID, game);
	});

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="ZigStore Homepage" />
</svelte:head>

{#await store.get("game"+props.gameID)}
    <p class="w-full text-gray-400 text-center text-2xl pt-8">Loading Game Info...</p>		
{:then game} 
    <p class="text-xl text-gray-400 font-light pb-2 mb-2 mt-4 border-b border-white border-opacity-30 mx-16">{game.name}</p>
    <div class="sm:flex flex-wrap mx-16 pb-8 justify-around">			
    </div>    
{/await}