<script>
    import FileSelector from "$lib/common/FileSelector.svelte";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { filesStore, submitTemplate, templateStore } from "$lib/stores/questions";
    let { proceed = $bindable() } = $props();

    $effect(() => {
        proceed = ($filesStore?.length ?? 0) > 0;
    });
</script>

<div class="select-none flex flex-col gap-5">
    <div class="glass p-10">
        <p class="text-xl font-semibold">File Upload</p>
        <div class="py-2 opacity-60">Thank you for choosing to contribute in our research project. Please select files that you'd like to contribute to our system.</div>
        <div class="py-1 opacity-80">Help us improve our search by contributing. Contribution is a few-step process where you answer some questions about your files.</div>
    </div>
    <div class="grid glass p-5 gap-3">
        <div class="p-3 glass grid text-center grid-cols-[1fr_50px_1fr] gap-10 items-center justify-center">
            <div class="grid gap-3">
                <div class="text-primary/50">Upload files individually.</div>
                <div class="flex items-center justify-center"><FileSelector text="Select files" multiple={true} bind:files={$filesStore} /></div>
            </div>
            <div class="uppercase opacity-60 rounded border-y bg-secondary/20 border-secondary/60 h-full w-0.5 m-auto"></div>
            <div class="grid gap-3 items-center m-auto">
                <div class="text-primary/50">Fill out an excel file and upload it for us to process it later.</div>
                <a target="_blank" href="../template.xlsm" class="text-nowrap glass px-2 py-1 text-green-600/80 glass bg-green-600/10 border-green-600/50 flex gap-3 items-center justify-center hover:brightness-[90%] cursor-pointer">
                    <div class="glass p-2 rounded-full bg-white/80 border-green-600/30">
                        <MaskedIcon src="../download.svg" class="size-4 bg-green-600" />
                    </div>
                    <div class="text-center px-5">Download template</div>
                </a>
                <FileSelector class="w-full" text={$templateStore?.item(0)?.name ?? "Upload excel file"} multiple={false} bind:files={$templateStore} accept=".xlsm" />
                {#if ($templateStore?.length ?? 0) > 0}
                    <button
                        class="bg-primary p-3 rounded-2xl text-white"
                        onclick={async () => {
                            const files = $templateStore;
                            if (files) {
                                await submitTemplate(files);
                                templateStore.set(undefined);
                            }
                        }}>Submit</button
                    >
                {/if}
            </div>
        </div>
        <div class="glass p-10">
            {#if ($filesStore?.length ?? 0) > 0}
                <div class="font-light pb-2">Selected files ({$filesStore?.length ?? 0}):</div>
                {@const fileNames = Array.from($filesStore ?? []).map((x) => x.name)}
                <ul class="space-y-2 bg-secondary/5 border border-secondary/50 shadow shadow-secondary/10 py-3 px-5 rounded-lg">
                    {#each fileNames as fileName}
                        <li class="flex items-center gap-3">
                            <MaskedIcon src="../checkmark-circle.svg" class="size-3 bg-primary" />
                            {fileName}
                        </li>
                    {/each}
                </ul>
            {:else}
                <div class="text-center font-light opacity-80 italic">No files selected</div>
            {/if}
        </div>
    </div>
</div>
