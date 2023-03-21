<script lang="ts">
	import type { SteamGame } from "$lib/types/Steam";
    export let props: { installGame: SteamGame | null };
    let installPath: string = 'C:\\Games\\'+props.installGame!.name+'\\';

</script>

{#if props.installGame}
<div class={`${props.installGame ? 'absolute' : 'hidden'} top-0 left-0 w-full h-full bg-black bg-opacity-30 backdrop-blur-md flex justify-center items-center`}>
    <div class="border border-gray-400 border-opacity-30 bg-[#0a0a0a] p-2 text-gray-400 rounded-lg w-96 px-4">
        <p class="w-full border-b border-white border-opacity-30 mb-2 pb-1">
            Installing {props.installGame.name}
        </p>
        <input type="text" bind:value={installPath} class="w-full bg-[#131313] pl-4 rounded-lg text-lg" placeholder="Install Path" />
        <div class="flex">
            <p class="text-xs w-10/12 px-2">You need at least {props.installGame.internal.size.toLocaleString()}GB available to perform this action!</p>
            <button class="w-24 hover:brightness-125 rounded float-right mr-2 mt-2 bg-[#1a1a1a] border border-white border-opacity-30" on:click={() => console.log('ok')}>Install</button>
        </div>

    </div>    
</div>
{/if}