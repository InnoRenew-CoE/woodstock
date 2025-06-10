<script lang="ts">
    import MaskedIcon from "./MaskedIcon.svelte";

    let { files = $bindable() }: { files: FileList | undefined } = $props();
</script>

<div class="flex items-center justify-center p-5">
    <label for="file-upload" class="glass flex gap-3 p-2 py-1 items-center justify-center hover:brightness-[99%] cursor-pointer">
        <div class="glass p-3">
            <MaskedIcon src="../sticker-circle.svg" class="size-4 bg-secondary" />
        </div>
        <div class="text-center px-5">Select files</div>
    </label>
    <input id="file-upload" class="hidden" type="file" bind:files multiple />
</div>
