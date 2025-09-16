<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import { marked } from "marked";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import type { ResultChunk } from "$lib/types/question";
    import Spacer from "$lib/common/Spacer.svelte";
    import { pushNotification } from "$lib/stores/notifications";

    let data: string = $state("");
    let query: string = $state("");
    let waiting = $state(false);
    let component: HTMLDivElement | undefined = $state(undefined);
    let maxHeight = $derived((component?.clientHeight ?? 100) * 0.8);
    let chunks: ResultChunk[] = $state([]);

    // chunks = [
    //     {
    //         id: 'PointId { point_id_options: Some(Uuid("c0f02548-4da3-4fcd-82bf-c63dadbb445a")) }',
    //         doc_id: "2",
    //         doc_seq_num: 7,
    //         content:
    //             "vector space. When a user query q arrives, it is embedded into q = f ( q ) . The system then locates the nearest v ij vectors within E , and retrieves the associated chunks for nal answer generation by an LLM. Although the pipeline remains structurally similar to a Naive RAG, the key difference is that HyPE matches questions against questions, rather than questions against chunk text. This questionquestion alignment increases the probabil- ity of nding the correct chunks for two main reasons. First, many embedding models exhibit style-based clustering [13]. Texts of similar form (e.g., interrogative sentences) often lie closer in the vector space. As a result, a user’s real-world query naturally aligns more closely with the hypothetical prompts that share its interrogative style. Second, generating multiple hypothetical queries per chunk broadens the se- mantic reach, covering a wider range of possible question formulations. Even if a user query is phrased in a slightly different way, there is a higher chance that at least one of the chunk’s hypothetical questions closely corresponds to it. Another advantage of HyPE lies in how it addresses the inherent chunking tradeoff in retrieval systems. If chunks are too large, their embeddings become less precise because they encode a mix of multiple concepts, making vector-based similarity less reliable [14]. Conversely, reducing chunk size improves embedding specicity but risks losing crucial sur- rounding context. HyPE mitigates this issue by ensuring that each stored vector represents a specic piece of information within a chunk,",
    //         additional_data: "What is HyPE?",
    //         score: 0.687968,
    //     },
    //     {
    //         id: 'PointId { point_id_options: Some(Uuid("1e717a2c-fab1-4460-b8c0-03f64f0bbbff")) }',
    //         doc_id: "2",
    //         doc_seq_num: 5,
    //         content:
    //             "time and, instead, alters how we store the passages (i.e., their hypothetical question embeddings). More recently, HyDE [5] addresses querydocument mis- match by generating a hypothetical answer or short passage at query time. Instead of embedding the user’s question directly, HyDE prompts an LLM to produce an approximate response, then embeds that synthetic text. This is used to retrieve relevant real documents from a vector index. While HyDE can improve retrieval accuracy for zero-shot question answering, it incurs an extra inference cost per user query. Additionally, the method may struggle, where the prompt queries for niche domain knowledge, where the model may not have sufcient knowledge to produce a representative sample. III. METHODOLOGY HyPE addresses the challenge of aligning user queries and relevant content by pre-computing hypothetical prompts at the indexing stage, contrasting with HyDE’s runtime genera- tion of synthetic answers. This shift avoids additional infer- ence overhead per query and improves retrieval precision by ensuring that both user queries and stored embeddings share a question-like form. The method begins by splitting the corpus D into coherent chunks C 1 ; C 2 ; : : : ; C n , where each chunk provides a self-contained unit of information. For each chunk C i , an LLM G generates multiple hypothetical prompts Q i = q i 1 ; q i 2 ; : : : ; q ik , simu- lating possible user queries that the chunk might answer. This ofine step does not introduce any additional computational",
    //         additional_data: "What is the main idea of HyPE according to the passage?",
    //         score: 0.65756434,
    //     },
    // ];

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
            data += decoded;
            total++;
            try {
                chunks = JSON.parse(data);
                data = "";
            } catch (e) {}
            // if (total === 1) {
            //     chunks = JSON.parse(decoded);
            //     continue;
            // }
            if (done) {
                console.log("Done");
                break;
            }
        }
        const datas = data.split("</think>");
        const think = datas[0];
        const notThink = datas[1];
        // data = notThink;
        waiting = false;
    }

    async function vote() {
        pushNotification({ title: "Vote received", body: "Thank you for rating the information provided by our system." });
    }
</script>

<div class="flex flex-wrap sm:grid grid-cols-2 h-full gap-5 min-h-[80vh] glass p-3">
    <div class="p-5 glass w-full">
        <form class="flex gap-5 items-stretch">
            <input bind:value={query} type="text" class="w-full py-2 px-4 glass border-1 rounded-xl placholder:text-accent" placeholder="Ask a question ..." />
            <button type="submit" onclick={sendQuery} class="glass rounded-xl px-5 flex-1 flex gap-3 items-center hover:bg-secondary/10">
                <MaskedIcon src="../contact.svg" class="size-3 bg-secondary" />
                Ask
            </button>
        </form>
        {#if chunks.length > 0}
            <div class="pt-5 pl-5 opacity-50 font-mono text-xs">Data retrieved from files:</div>
        {/if}

        <ul class="grid gap-5 py-3">
            {#each chunks.slice(0, 5) as chunk, i}
                <li class="glass p-3" in:slide={{ delay: i * 1000 }}>
                    <div>
                        <div class="p-3 flex gap-3 items-center">
                            <div class="flex gap-2">
                                <div class="text-secondary bg-secondary/5 border-secondary/50 p-2 shadow-secondary/30 glass rounded-full font-mono text-xs">#{i + 1}</div>
                            </div>
                            <div class="flex-1 flex gap-2">
                                <div class="py-1 px-3 text-xs h-min glass bg-secondary/10 border border-secondary/40 rounded-sm text-secondary">pdf</div>
                            </div>
                            <div class="flex gap-2">
                                Score
                                <div class="py-1 px-3 text-xs h-min glass bg-green-700/10 border border-green-700/40 rounded-sm text-green-700">high</div>
                            </div>
                            {#if (parseInt(chunk.doc_id) ?? 0) > 0}
                                <a target="_blank" href="/api/download/{chunk.doc_id}" class="disabled:opacity-50 disabled:!cursor-no-drop glass px-3 py-2 flex gap-2 items-center">
                                    <MaskedIcon src="../download.svg" class="size-3 bg-secondary group-hover:bg-accent/50" />
                                </a>
                            {/if}
                        </div>
                        <div class="">
                            <div class="transition-all response preview p-3 prose-sm glass text-sm">
                                <div class="flex gap-3 items-center pb-3">
                                    <div class="uppercase text-accent/50 font-mono text-xs pb-2">Preview</div>
                                    <Spacer />
                                    <div class="glass p-1 flex gap-3 items-center rounded-full">
                                        <button onclick={vote} class=" hover:brightness-110 active:brightness-90 hover:bg-green-500/5 hover:border-green-500 glass p-2 rounded-full"><MaskedIcon src="../thumbs-up.svg" class="bg-green-500" /></button>
                                        <button onclick={vote} class=" hover:brightness-110 active:brightness-90 hover:bg-red-500/5 hover:border-red-500 glass p-2 rounded-full"><MaskedIcon src="../thumbs-down.svg" class="bg-red-500" /></button>
                                    </div>
                                </div>
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
            <div class="text-center text-accent glass p-5">You haven't asked or queried for anything yet.</div>
        {/if}
    </div>
</div>

<style>
    .response :global(li) {
        padding: 0.25rem 0;
        transition: all 0.2s ease-in-out;
    }
</style>
