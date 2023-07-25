<script>
	import Home from '$components/Pages/Home.svelte';
	import Install from '$components/Modal/install.svelte';
	import Queue from '../components/Pages/Queue.svelte';

	$: page = window.location.toString().includes('page=queue') ? Queue : Home;
	$: props = {};
</script>

<svelte:component this={page} bind:props bind:page />
<Install bind:props />
