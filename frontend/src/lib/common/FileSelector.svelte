<script lang="ts">
    import MaskedIcon from "./MaskedIcon.svelte";

    let { files = $bindable() }: { files: FileList | undefined } = $props();
</script>

<div class="flex items-center justify-center p-5">
    <label for="file-upload" class="flex items-center gap-3 border border-accent bg-accent/5 text-accent w-min whitespace-nowrap px-5 py-2 rounded hover:brightness-75 cursor-pointer">
        <MaskedIcon src="../paperclip.svg" class="size-4 bg-accent" />
        Select files
    </label>
    <input id="file-upload" class="hidden" type="file" bind:files multiple />
</div>
