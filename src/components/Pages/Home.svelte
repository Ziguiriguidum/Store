
<script>
	import { onMount, getContext } from 'svelte';
	import { Store } from "tauri-plugin-store-api";	
	import Game from './Game.svelte';
	const store = new Store(".settings.dat");

	export let props;
	export let page;
	
	onMount(async () => {
		const gameList = await (await fetch('https://store.caiocinel.com/api/list')).json();
		if(gameList.games.length > 0){
			store.set("gameList", gameList.games);
			store.save();
		}
	});

	const loadGame = (game) =>{
		props.gameID = game.id;		
		page = Game;
	}

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="ZigStore Homepage" />
</svelte:head>

{#await store.get("gameList")}
	<p class="w-full text-gray-400 text-center text-2xl pt-8">Loading Game List...</p>				
{:then games} 
	{#if games.length === 0}
		<p class="w-full text-gray-400 text-center text-2xl pt-8">No Games Found</p>
	{:else}
		<p class="text-xl text-gray-400 font-light pb-2 mb-2 mt-4 border-b border-white border-opacity-30 mx-16">Ziguiriguidum Store</p>
		<div class="sm:flex flex-wrap mx-16 pb-8 justify-around">
		{#each games as game}
			<button on:click={() => {loadGame(game)}} class="pb-4 hover:scale-125 hover:text-white brightness-90 hover:brightness-110 cursor-pointer transition p-2 hover:bg-[#1f1f1f] hover:z-50 rounded">
				<img width="256px" src={`https://cdn.akamai.steamstatic.com/steam/apps/${game.id}/header.jpg`} alt="icon" />
				<p class="text-gray-400 text-xs w-64 whitespace-nowrap text-ellipsis">{game.name}{game.version !== null ? ' - '+game.version.substring(0, 10).replace(/\.$/, "...") : ''} - {Number(game.size).toLocaleString()}GB</p>
			</button>
		{/each}				
		</div> 
	{/if}
{/await}