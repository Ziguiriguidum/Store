<script>
    import { setContext, getContext } from 'svelte';
	import Home from '$components/Pages/Home.svelte';

    if(!getContext('page')) {
        setContext('page', Home);
    }
</script>

<svelte:component this={getContext('page')} />