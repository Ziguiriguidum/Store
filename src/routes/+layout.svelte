<script>
	import './styles.css';
	import WindowMenu from '../components/WindowMenu.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Store } from "tauri-plugin-store-api";

	const store = new Store(".settings.dat");
	
</script>
 
<div class="app">
	<WindowMenu />	
	<div class="flex h-[97vh]">
		<div class="w-2/12 h-full border-r border-white border-opacity-30 ">
			<p class="mt-2 text-sm text-center text-gray-400">Your Library</p>
			<hr class="opacity-50 my-2"/>						
			{#await store.get("mygames")}
				<p class="text-sm text-center text-gray-400">Loading...</p>
			{:then games}
				{#if !games || games.length === 0}
					<p class="text-sm text-center text-gray-400">No games</p>
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

		</div>
	</div>	
	<slot />
</div>