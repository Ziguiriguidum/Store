<script>
	import './styles.css';
	import WindowMenu from '../components/WindowMenu.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	
</script>
 
<div class="app">
	<WindowMenu />	
	<div class="flex h-[97vh]">
		<div class="w-2/12 h-full border-r border-white border-opacity-30 ">
			<p class="mt-2 text-sm text-center text-gray-400">Your Library</p>
			<hr class="opacity-50 my-2"/>			

		</div>
		<div class="w-full">

		</div>
	</div>	
	<slot />
</div>