<script>
	import './styles.css';
	import WindowMenu from '../components/WindowMenu.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Store } from "tauri-plugin-store-api";
	import { onMount } from 'svelte';
	const store = new Store(".settings.dat");

	onMount(async () => {
		const gameList = await (await fetch('https://store.caiocinel.com/api/list')).json();
		if(gameList.games.length > 0)
			store.set("gameList", gameList.games);
	});
 
	// https://store.steampowered.com/api/appdetails?appids={APP_ID}&l=english
	// https://cdn.akamai.steamstatic.com/steam/apps/{APP_ID}/header.jpg
	
</script>
 
<div class="app">
	<WindowMenu />	
	<div class="flex h-[97vh]">
		<div class="w-2/12 border-r border-white border-opacity-30">
			<p class="mt-2 text-sm text-center text-gray-400">Your Library</p>
			<hr class="opacity-50 my-2"/>						
			{#await store.get("myGames")}
				<p class="text-sm text-center text-gray-400">Loading...</p>
			{:then games}
				{#if !games || games.length === 0}
					<p class="text-sm text-center text-gray-400">No Games Found</p>
				{:else}
					{#each games as game}
					<div class="flex flex-row items-center justify-between px-2 py-1 hover:bg-white hover:bg-opacity-10">
						<div class="flex flex-row items-center">
							<img src={game.icon} class="w-6 h-6" alt="icon" />
							<p class="ml-2 text-sm">{game.name}</p>
						</div>					
					</div>
				{/each}
				{/if}
			{/await}			
		</div>
		<div class="w-full">
			{#await store.get("gameList")}
				<p>Loading Game List...</p>				
			{:then games} 
				<p class="text-xl text-gray-400 font-light pb-2 mb-2 mt-4 border-b border-white border-opacity-30 mx-16">Ziguiriguidum Store</p>
				<div class="sm:flex flex-wrap mx-16 pb-8 justify-around">
				{#each games as game}
					<div>
						<img width="256px" src={`https://cdn.akamai.steamstatic.com/steam/apps/${game.id}/header.jpg`} alt="icon" />
						<p class="text-gray-400 text-xs w-64 whitespace-nowrap text-ellipsis">{game.name}{game.version !== null ? ' - '+game.version.substring(0, 10).replace(/\.$/, "...") : ''} - {Number(game.size).toLocaleString()}GB</p>
					</div>
				{/each}				
				</div> 
			{/await}
		</div>
	</div>	
	<slot />
</div>