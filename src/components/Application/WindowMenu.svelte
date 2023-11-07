<script>
    import {getCurrent} from "@tauri-apps/api/window"
</script>

<div data-tauri-drag-region class="h-6 select-none flex justify-end fixed top-0 right-0 left-0 bg-black bg-opacity-50 border-b border-white border-opacity-30 z-50">
    <button on:click={async() => getCurrent().minimize()} class="w-8 hover:bg-white hover:bg-opacity-30 brightness-125">
        <img src="https://api.iconify.design/mdi:window-minimize.svg" class="invert mx-auto" alt="minimize" />
    </button>
    <button on:click={() => getCurrent().close()} class="w-8 hover:bg-red-500 hover:bg-opacity-70">
        <img src="https://api.iconify.design/mdi:close.svg" class="invert mx-auto" alt="close" />
    </button>
</div>