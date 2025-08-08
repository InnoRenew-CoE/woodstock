<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { filesStore } from "$lib/stores/questions";
    import { uploadProgressStore } from "$lib/stores/uploads";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    let { step = $bindable(), files = $bindable() }: { step: number; files: FileList | undefined } = $props();

    function reset() {
        step = 0;
        files = undefined;
    }
</script>

<div class="flex justify-center p-5">
    <div class="w-full">
        <img src="../tree_of_knowledge.svg" alt="Tree of knowledge" class="size-36 my-5 mx-auto rounded-full bg-secondary/5 p-5 shadow-lg shadow-secondary/30 border border-secondary" />
        <div class="text-lg font-bold">Submission</div>
        <div class="font-light">Thank you for answering our questions. Your files and answers are being uploaded and safely stored.</div>

        <div class="glass p-5 my-5">
            <span class="font-mono text-xs uppercase">Progress</span>
            <ul class="space-y-2">
                {#each $uploadProgressStore as upload}
                    <li>
                        <div class="flex gap-3">
                            <div class="truncate w-[200px]">{upload.file}</div>
                            <div class="w-full bg-gray-50 border border-primary/10 glass overflow-hidden rounded-sm">
                                <div class="bg-secondary/80 w-[0%] h-full transition-all glass rounded-sm" style="width: {upload.progress}%"></div>
                            </div>
                            <div class="w-[20%] text-right font-light">{upload.progress.toFixed(2)}%</div>
                        </div>
                    </li>
                {/each}
            </ul>
        </div>

        {#if $uploadProgressStore.every((f) => f.progress >= 100)}
            <div in:fade>
                <div class="w-full border-b border-accent/10 mb-5"></div>
                <span class="font-nunito">You can safely close the tab or continue to upload more files.</span>
                <button class="px-3 py-2 rounded my-3 flex items-center gap-2 justify-center bg-secondary text-white" onclick={reset}>Upload more</button>
            </div>
        {/if}
    </div>
</div>
