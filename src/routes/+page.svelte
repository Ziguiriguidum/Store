<script>
	import Home from '$components/Pages/Home.svelte';
	import Install from '$components/Modal/install.svelte';

	$: page = Home;
	$: props = {};
</script>

<svelte:component this={page} bind:props bind:page />
<Install bind:props />
