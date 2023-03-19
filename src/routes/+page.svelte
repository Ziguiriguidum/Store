<script>
	import Home from '$components/Pages/Home.svelte';
    import Install from '$components/Modal/install.svelte';
    
    $: page = Home;
    $: props = {};
    
</script>

<svelte:component bind:props={props} bind:page={page} this={page}></svelte:component>
<Install bind:props={props} />