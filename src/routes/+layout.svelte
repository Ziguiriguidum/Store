<script>
	import './styles.css';
	import { appWindow } from '@tauri-apps/api/window';
</script>

<div class="app">
	<div on:mousedown={() => appWindow.startDragging()} class="h-6 select-none flex justify-end fixed top-0 right-0 left-0 bg-black bg-opacity-50">
		<button on:click={() => appWindow.minimize()} class="w-8 hover:bg-white hover:bg-opacity-30 brightness-125">
			<img src="https://api.iconify.design/mdi:window-minimize.svg" class="invert mx-auto" alt="minimize" />
		</button>
		<button on:click={() => appWindow.close()} class="w-8 hover:bg-red-500 hover:bg-opacity-70">
			<img src="https://api.iconify.design/mdi:close.svg" class="invert mx-auto" alt="close" />
		</button>
	</div>
	<slot />
</div>