<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { onMount } from "svelte";

    let files: {
        name: string;
        date: Date;
        file_type: string;
        tags: string[];
    }[] = $state([]);
    onMount(async () => {
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/files`);
        files = await response.json();
        files.sort((a, b) => a.name.localeCompare(b.name));
        for (let f of files) {
            f.date = new Date(f.date);
        }
    });
</script>

<div class="glass p-5">
    <div class="w-full grid gap-3">
        <!-- <div id="header" class="grid gap-3 grid-cols-[1fr_1fr_10%_5%] text-base px-1">
            <div class="glass hover:bg-white/60 px-4 py-1">File</div>
            <div class="glass hover:bg-white/60 px-4 py-1">Tags</div>
            <div class="glass hover:bg-white/60 px-4 py-1 text-center">Date</div>
            <div class="glass hover:bg-white/60 px-4 py-1 text-center">Type</div>
        </div> -->
        <div id="body" class="glass py-5 grid max-h-[600px] overflow-auto px-2 text-sm">
            {#each files as file, i}
                <div class="transition-all grid grid-cols-[1fr_20%_10%_5%] gap-x-3 rounded-lg hover:bg-secondary/5 hover:border-secondary/50 border border-transparent py-2">
                    <div class="px-10 flex items-center text-base">{file.name}</div>
                    <div class="glass p-1 rounded-full flex items-center gap-3 max-w-[100%] overflow-x-scroll no-scrollbar">
                        {#each file.tags as tag}
                            <div class="glass px-3 py-1 bg-secondary/5 text-secondary border-secondary/50">{tag}</div>
                        {/each}
                    </div>
                    <div class="flex justify-end items-center font-mono text-xs">
                        <div class="glass w-min px-3 py-1 bg-indigo-100/50 text-indigo-500 border-indigo-200 whitespace-nowrap">
                            {file.date.toLocaleDateString()}
                        </div>
                    </div>
                    <div class="flex items-center justify-center font-mono text-xs">
                        <div class="glass w-min px-3 py-1 shadow-2xs bg-amber-100/50 text-amber-600 border-amber-200 whitespace-nowrap lowercase">
                            {file.file_type}
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    th,
    td {
        text-align: left;
        padding: 0.25rem;
    }
</style>
