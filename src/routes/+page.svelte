<script>
	import { invoke } from '@tauri-apps/api/tauri';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="text-5xl">
	<button on:click={()=> invoke('my_custom_command')}>Teste</button>
</div>