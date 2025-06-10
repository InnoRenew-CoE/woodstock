<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import { marked } from "marked";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import type { ResultChunk } from "$lib/types/question";

    let data: string = $state("");
    let query: string = $state("");
    let waiting = $state(false);
    let component: HTMLDivElement | undefined = $state(undefined);
    let maxHeight = $derived((component?.clientHeight ?? 100) * 0.8);
    let chunks: ResultChunk[] = $state([]);

    async function sendQuery() {
        data = "";
        chunks = [];
        waiting = true;
        console.log(`${PUBLIC_API_BASE_URL}/api/search?query=${query}`);
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/search?query=${query}`);
        const stream = response.body?.getReader();

        if (!stream) {
            console.error("idk bro ");
            return;
        }
        const decoder = new TextDecoder("utf-8");
        let total = 0;
        while (true) {
            const { done, value } = await stream?.read();
            const decoded = decoder.decode(value);
            total++;
            if (total === 1) {
                chunks = JSON.parse(decoded);
                continue;
            }
            if (done) {
                console.log("Done");
                break;
            }

            data += decoded;
        }
        waiting = false;
    }
</script>

<div class="flex flex-wrap sm:grid grid-cols-2 h-full gap-5 min-h-[80vh] glass p-3">
    <div class="p-5 glass w-full">
        <form class="flex gap-5 items-stretch">
            <input bind:value={query} type="text" class="w-full py-2 px-4 glass border-2" placeholder="Ask a question ..." />
            <button type="submit" onclick={sendQuery} class="glass px-5 flex-1 flex gap-3 items-center hover:bg-secondary/10">
                <MaskedIcon src="../contact.svg" class="size-3 bg-secondary" />
                Ask
            </button>
        </form>
        {#if chunks.length > 0}
            <div class="pt-5 text-gray-500">Data retrieved from files:</div>
        {/if}

        <ul class="spacing-y-5 py-5">
            {#each chunks.slice(0, 5) as chunk, i}
                <li class="bg-secondary/5 group shadow-secondary/30 border-secondary/30 mb-5 hover:border-accent hover:shadow-accent/50 hover:bg-accent/5 border p-3 rounded-lg" in:slide={{ delay: i * 1000 }}>
                    <div>
                        <div class="flex justify-between items-center bg-white py-2 px-5 rounded-lg border">
                            <div class="flex gap-2">
                                <div class="text-xs bg-secondary/5 px-2 rounded border-secondary/50 border group-hover:border-accent/50 group-hover:bg-accent/5 group-hover:text-accent">#{i + 1}</div>
                                <div>{chunk.additional_data}</div>
                            </div>
                            <button disabled class="disabled:opacity-50 bg-gray-50 border py-1 px-2 rounded flex items-center gap-2">
                                <MaskedIcon src="../download.svg" class="size-3 bg-secondary group-hover:bg-accent/50" />
                                Download
                            </button>
                        </div>
                        <div class="text-wrap text-xs p-2 truncate bg-light-background border rounded-lg mt-3">
                            <div class="uppercase text-gray-400">Preview</div>
                            <div class="response preview p-3 prose-sm text-xs">
                                <div class="">{@html marked("... " + chunk.content.slice(0, 3000) + " ...")}</div>
                            </div>
                        </div>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
    <div id="llm" class=" p-10 glass w-full" bind:this={component}>
        {#if data && data.length >= 0}
            <div in:fade>
                <div class="opacity-30">Woody's response</div>
                <div class="overflow-auto p-5 flex flex-col-reverse">
                    <div class="response preview spacing-y-2 prose-sm">{@html marked(data)}</div>
                </div>
            </div>
        {:else if waiting}
            <div class="flex items-center justify-center gap-5">
                <MaskedIcon src="../loading.svg" class="size-3 bg-secondary animate-spin" />
                Waiting for data!
            </div>
        {:else}
            <div class="text-center text-gray-500">You haven't asked or queried for anything yet.</div>
        {/if}
    </div>
</div>

<style>
    .response :global(li) {
        padding: 0.25rem 0;
        transition: all 0.2s ease-in-out;
    }
</style>
