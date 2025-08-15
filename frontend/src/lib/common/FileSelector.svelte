<script lang="ts">
    import MaskedIcon from "./MaskedIcon.svelte";

    let { text = "Select files", files = $bindable(), multiple = true }: { text: string; files: FileList | undefined; multiple: boolean } = $props();
</script>

<div class="w-min text-nowrap">
    <label for="file-upload" class="text-secondary/80 glass bg-secondary/10 border-secondary/50 flex gap-3 p-2 py-1 items-center justify-center hover:brightness-[90%] cursor-pointer">
        <div class="glass p-2 rounded-full bg-white/70">
            <MaskedIcon src="../download.svg" class="size-4 bg-secondary rotate-180" />
        </div>
        <div class="text-center px-5">{text}</div>
    </label>
    <input id="file-upload" class="hidden" type="file" bind:files {multiple} />
</div>
