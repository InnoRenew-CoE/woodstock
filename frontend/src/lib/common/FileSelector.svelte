<script lang="ts">
    import MaskedIcon from "./MaskedIcon.svelte";

    let { files = $bindable() }: { files: FileList | undefined } = $props();
</script>

<div class="flex items-center justify-center p-5">
    <label for="file-upload" class="flex items-center rounded border border-secondary overflow-hidden group bg-secondary/10 hover:bg-secondary cursor-pointer hover:text-white">
        <div class="bg-secondary p-2 group-hover:bg-white">
            <MaskedIcon src="../sticker-circle.svg" class="size-4 bg-white group-hover:bg-secondary" />
        </div>
        <div class="text-center px-5">Select files</div>
    </label>
    <input id="file-upload" class="hidden" type="file" bind:files multiple />
</div>
