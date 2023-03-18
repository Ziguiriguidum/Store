<script>
	import Home from '$components/Pages/Home.svelte';
    
    $: page = Home;
    $: props = {};
    
</script>

<svelte:component {...props} bind:props={props} bind:page={page} this={page} />