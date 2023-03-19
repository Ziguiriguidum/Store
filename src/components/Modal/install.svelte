<script lang="ts">
    export let props;
</script>

<div hidden={!props.installGameId} class="absolute top-0 left-0 w-full h-full bg-black bg-opacity-30 backdrop-blur-md flex justify-center items-center">
    <div class="border border-gray-400 border-opacity-30 bg-[#0a0a0a] p-2 text-gray-400 rounded-lg">
        Installing Game
    </div>
    
</div>