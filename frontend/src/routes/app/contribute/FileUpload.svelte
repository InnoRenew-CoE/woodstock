<script>
    import FileSelector from "$lib/common/FileSelector.svelte";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { filesStore } from "$lib/stores/questions";
    let { proceed = $bindable() } = $props();

    $effect(() => {
        proceed = ($filesStore?.length ?? 0) > 0;
    });
</script>

<div class="select-none">
    <p class="text-xl font-semibold">File Upload</p>
    <div class="py-2 opacity-60">Thank you for choosing to contribute in our research project. Please select files that you'd like to contribute to our system.</div>
    <div class="py-1 opacity-80">Help us improve our search by contributing. Contribution is a few-step process where you answer some questions about your files.</div>
    <div>
        <div><FileSelector bind:files={$filesStore} /></div>
        {#if ($filesStore?.length ?? 0) > 0}
            <div class="font-light pb-2 text-accent">Selected files:</div>
            {@const fileNames = Array.from($filesStore ?? []).map((x) => x.name)}
            <ul class="px-3">
                {#each fileNames as fileName}
                    <li class="flex items-center gap-2">
                        <MaskedIcon src="../checkmark-circle.svg" class="size-3 bg-primary/50" />
                        {fileName}
                    </li>
                {/each}
            </ul>
        {:else}
            <div class="text-center font-light opacity-80 italic">No files selected</div>
        {/if}
    </div>
</div>
