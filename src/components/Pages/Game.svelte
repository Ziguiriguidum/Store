<script lang="ts">
	import { Store } from 'tauri-plugin-store-api';
	import type { SteamGame } from '$lib/types/Steam';
	import Carousel from 'svelte-carousel';
	import chevronDown from '$lib/images/chevron-down.svg';
	const store = new Store('.settings.dat');
	import bbobHTML from '@bbob/html'
	import presetHTML5 from '@bbob/preset-html5'

	export let props;
	export let page;

	//Just to stop unused warning
	page = page;

	const getGame = async () => {
		let data: SteamGame | null = (
			await (await fetch(`https://store.caiocinel.com/api/game?id=${props.gameID}`)).json()
		).data[Number(props.gameID)].data;
		if (data) await store.set('game' + props.gameID, data);
		else data = await store.get('game' + props.gameID);

		if (!data) throw new Error('Game not found');
		return data;
	};
</script>

{#await getGame()}
	<p class="w-full text-gray-400 text-center text-2xl pt-8">Loading Game Data...</p>
{:then game}
	<p
		class="text-xl text-gray-400 font-light pb-2 mb-2 mt-4 border-b border-white border-opacity-30 mx-16"
	>
		{game.name}
	</p>
	<div class="flex mx-16">
		<div class="w-[650px]">
			<Carousel autoplay arrows={false}>
				{#each game && game.screenshots as { id, path_full }}
					<img alt="Game Screenshot" src={path_full} />
				{/each}
			</Carousel>
		</div>
		<div class="w-full ml-4">
			<p class="text-xl">
				{game.name}<span class="text-xs"
					>&nbsp;{game.internal.version ? game.internal.version : ''}</span
				>
			</p>
			<p class="text-gray-400">{game.developers[0]}</p>
			<p class="text-gray-400">{game.publishers[0]}</p>
			<p class="text-gray-400">
				{game.price_overview ? game.price_overview.final_formatted : 'Free'} • {game.release_date
					.date} • {game.internal.platform}
			</p>
			<p class="text-gray-400">
				{Number(game.player_count).toLocaleString()} players • {Number(
					game.internal.size
				).toLocaleString()}GB
			</p>
			<p class="text-gray-400">
				{Number(game.recommendations.total).toLocaleString()} reviews (<span class="text-green-700"
					>{Number(game.reviews.query_summary.total_positive).toLocaleString()}</span
				>/<span class="text-red-700"
					>{Number(game.reviews.query_summary.total_negative).toLocaleString()}</span
				>)
			</p>
			<p class="mt-1">{game.short_description}</p>
			<div class="absolute top-[400px] w-80">
				<p class="text-gray-400">By: <strong>{game.internal.sceneGroup}</strong></p>
				<button on:click={()=> props.installGame = game} class="w-full bg-gray-600 rounded-full text-lg">Install</button>
			</div>
		</div>
	</div>
	<div class="flex gap-x-16 mx-16 text-xs h-60 overflow-hidden">
		<div class="w-72">
			{@html game.pc_requirements.minimum}
		</div>
		<div class="w-72">
			{@html game.pc_requirements.recommended}
		</div>
		<div class="flex gap-x-4">
			<div class="w-1/2">
				<strong class="leading-10">Genres</strong>
				{#each game.genres as genre}
					<p>{genre.description}</p>
				{/each}
			</div>
			<div class="w-1/2">
				<strong class="leading-10">Categories</strong>
				{#each game.categories as categ}
					<p>{categ.description}</p>
				{/each}
			</div>
			{#if game.metacritic}
				<div class="w-1/2">
					<strong class="leading-10">Metacritic</strong>
					<div
						class={`${game.metacritic.score > 74 && 'bg-[#66CC33]'} ${
							game.metacritic.score < 75 && game.metacritic.score > 49 && 'bg-[#FFCC33]'
						} ${
							game.metacritic.score < 50 && 'bg-[#FF0000]'
						} w-16 h-16 text-3xl text-center leading-[2]`}
					>
						{game.metacritic.score}
					</div>
				</div>
			{/if}
		</div>
	</div>
	<button class="w-full" on:click={()=> window.scrollTo(0, 800)}>
		<p class="text-center text-xs text-gray-400  h-8 leading-7 mt-14 border-gray-400">
			<img src={chevronDown} alt="Show more" class="inline-block w-4 h-4 invert" />
			Scroll to Read More
			<img src={chevronDown} alt="Show more" class="inline-block w-4 h-4 invert" />
		</p>
	</button>
	<div class="mx-16 mt-14">
		<p class="text-xl text-gray-400 border-b mb-4">Reviews</p>
	</div>
	{#each game.reviews.reviews as review}
	<div class="mx-16 mb-4 p-4 rounded-xl">
		{review.author.steamid} • {review.author.num_games_owned} games • {review.author.num_reviews} reviews • {(review.author.playtime_forever/60).toFixed(0)} hours played • {review.voted_up ? '✅' : '❌'}
		<div class="text-xs bg-gray-900 bg-opacity-50 ">{@html bbobHTML((review.review), presetHTML5())}</div>
		<hr/>
	</div>
	{/each}
{:catch}
	<p class="w-full text-gray-400 text-center text-2xl pt-8">Something goes wrong...</p>
{/await}
