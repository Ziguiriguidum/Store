<script>
	import './styles.css';
	import WindowMenu from '$components/Application/WindowMenu.svelte';
	import { Store } from 'tauri-plugin-store-api';
	const store = new Store('.settings.dat');
</script>

<div class={`app`}>
	<WindowMenu />
	<div class="grid grid-cols-12 h-[97vh]">
		<div class="col-span-2 border-r border-white border-opacity-30 h-full">
			<div class="fixed w-[212px] h-[97vh]">
				<button
					on:click={() => (window.location.href = '/')}
					class="mt-2 text-sm text-center w-full text-gray-400">Home</button
				>
				<hr class="opacity-50 my-2" />
				{#await store.get('myGames')}
					<p class="text-sm text-center text-gray-400">Loading...</p>
				{:then games}
					{#if !games || games.length === 0}
						<p class="text-sm text-center text-gray-400">No Games Found</p>
					{:else}
						{#each games as game}
							<div
								class="flex flex-row items-center justify-between px-2 py-1 hover:bg-white hover:bg-opacity-10"
							>
								<div class="flex flex-row items-center">
									<img src={game.icon} class="w-6 h-6" alt="icon" />
									<p class="ml-2 text-sm">{game.name}</p>
								</div>
							</div>
						{/each}
					{/if}
				{/await}
				<button
					class="absolute bottom-0 select-none hover:cursor-pointer hover:brightness-150 transition border-t border-white border-opacity-30 text-sm text-center w-full text-gray-400 py-2"
					on:click={() => (window.location.href = '/?page=queue')}
				>
					Downloads
				</button>
			</div>
		</div>
		<div class="col-span-10">
			<slot />
		</div>
	</div>
</div>
