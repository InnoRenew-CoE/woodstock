<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { onMount } from "svelte";

    type FileListing = { id: number; name: string };

    let files = $state<FileListing[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    async function load() {
        loading = true;
        error = null;
        try {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/api/public-files`);
            if (!response.ok) {
                throw new Error(`Request failed with status ${response.status}`);
            }
            files = (await response.json()) as FileListing[];
        } catch (err) {
            error = err instanceof Error ? err.message : "Unable to load files.";
        } finally {
            loading = false;
        }
    }

    onMount(load);
</script>

<div class="h-full min-h-0 flex flex-col gap-5">
    <header class="glass px-6 py-4 flex flex-wrap items-center justify-between gap-4">
        <div class="flex flex-col">
            <h1 class="text-2xl font-roboto leading-tight">Public files</h1>
            <p class="text-[11px] font-mono uppercase tracking-[0.2em] opacity-40">Download documents from the observatory</p>
        </div>
        <div class="flex items-center gap-3">
            <div class="glass bg-white/40 px-4 py-2 flex items-baseline gap-2">
                <span class="text-xl font-roboto leading-none">{files.length}</span>
                <span class="text-[10px] uppercase tracking-wider opacity-45">files</span>
            </div>
            <button
                class="flex items-center gap-2 glass bg-secondary/10 border-secondary/40 px-4 py-2 text-sm text-secondary transition-all hover:bg-secondary/20 disabled:opacity-50"
                onclick={load}
                disabled={loading}
            >
                <MaskedIcon src="/loading.svg" class="size-4 bg-secondary {loading ? 'animate-spin' : 'hidden'}" />
                {loading ? "Loading" : "Refresh"}
            </button>
        </div>
    </header>

    <div class="glass flex-1 min-h-0 p-5 flex flex-col">
        {#if loading && files.length === 0}
            <div class="flex flex-col gap-2">
                {#each [0, 1, 2, 3, 4, 5] as _}
                    <div class="h-[46px] animate-pulse rounded-lg bg-white/10"></div>
                {/each}
            </div>
        {:else if error}
            <div class="flex-1 flex flex-col items-center justify-center gap-3 text-center">
                <MaskedIcon src="/info.svg" class="size-8 bg-destructive/70" />
                <p class="font-roboto text-lg">Files are unavailable</p>
                <p class="text-sm opacity-50 max-w-md">{error}. Please make sure you are signed in and that the API is reachable.</p>
                <button class="mt-2 glass bg-white/40 px-5 py-2 text-sm hover:bg-white/60" onclick={load}>Try again</button>
            </div>
        {:else if files.length === 0}
            <div class="flex-1 flex flex-col items-center justify-center gap-3 text-center">
                <MaskedIcon src="/inbox.svg" class="size-8 bg-primary/40" />
                <p class="font-roboto text-lg">No files yet</p>
                <p class="text-sm opacity-50 max-w-md">Uploaded documents will show up here.</p>
            </div>
        {:else}
            <div class="flex-1 min-h-0 overflow-auto flex flex-col divide-y divide-primary/5 pr-1">
                {#each files as file}
                    <div class="flex items-center justify-between gap-4 px-3 py-2.5 rounded-lg transition-colors hover:bg-secondary/5">
                        <span class="truncate text-sm" title={file.name}>{file.name}</span>
                        <a
                            class="flex items-center gap-2 shrink-0 glass bg-secondary/10 border-secondary/40 px-4 py-1.5 text-sm text-secondary transition-all hover:bg-secondary/20"
                            href={`${PUBLIC_API_BASE_URL}/chat/download/${file.id}`}
                            title="Download {file.name}"
                        >
                            <MaskedIcon src="/download.svg" class="size-3.5 bg-secondary" />
                            Download
                        </a>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>
