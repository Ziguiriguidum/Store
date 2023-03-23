<script lang="ts">
	import type { SteamGame } from "$lib/types/Steam";
    import { open } from "@tauri-apps/api/dialog";
    export let props: { installGame: SteamGame | null } | any;    

    let installPath = 'C:\\Games\\';

</script>

{#if props.installGame}
<button class={`${props.installGame ? 'absolute' : 'hidden'} top-0 cursor-default left-0 w-full h-full bg-black bg-opacity-30 backdrop-blur-md flex justify-center items-center`}>
    <button on:click={(e) => e.preventDefault()} class="border border-gray-400 cursor-default border-opacity-30 bg-[#0a0a0a] p-2 text-gray-400 rounded-lg w-96 px-4">
        <div class="flex border-b border-white border-opacity-30 mb-2 pb-1 text-left">
            <p class="w-full">
                Installing {props.installGame.name}
            </p>
            <button class="float-right hover:brightness-125" on:click={() => props.installGame = null}>
                X
            </button>
        </div>
        <button on:click={() => (open({directory: true, defaultPath: installPath || "C:\\"}).then((e) => {e ? installPath = e.toString() : ''}))} class="absolute translate-x-80 brightness-[10] hover:brightness-[3] cursor-pointer">📁</button>
        <input type="text" bind:value={installPath} class="w-full bg-[#131313] pl-4 rounded-lg text-lg" placeholder="Install Path" />
        <div class="flex text-left">
            <p class="text-xs w-10/12 px-2">You need at least {props.installGame.internal.size.toLocaleString()}GB available to perform this action!</p>
            <button class="w-24 hover:brightness-125 rounded float-right mr-2 mt-2 bg-[#1a1a1a] border border-white border-opacity-30" on:click={() => console.log('ok')}>Install</button>
        </div>

    </button>    
</button>
{/if}