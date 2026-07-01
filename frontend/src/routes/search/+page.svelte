<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { pushNotification } from "$lib/stores/notifications";
    import { marked } from "marked";
    import { fade } from "svelte/transition";
    import { AudioRecorder } from "./AudioRecorder";

    interface Message {
        sender: string;
        data: string;
    }

    let activeQuery = $state("");
    let conversation = $state<Message[]>([]);

    let waiting = $state(false);

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
    //             "time and, instead, alters how we store the passages (i.e., their hypothetical question embeddings). More recently, HyDE [5] addresses querydocument mis- match by generating a hypothetical answer or short passage at query time. Instead of embedding the user's question directly, HyDE prompts an LLM to produce an approximate response, then embeds that synthetic text. This is used to retrieve relevant real documents from a vector index. While HyDE can improve retrieval accuracy for zero-shot question answering, it incurs an extra inference cost per user query. Additionally, the method may struggle, where the prompt queries for niche domain knowledge, where the model may not have sufcient knowledge to produce a representative sample. III. METHODOLOGY HyPE addresses the challenge of aligning user queries and relevant content by pre-computing hypothetical prompts at the indexing stage, contrasting with HyDE's runtime genera- tion of synthetic answers. This shift avoids additional infer- ence overhead per query and improves retrieval precision by ensuring that both user queries and stored embeddings share a question-like form. The method begins by splitting the corpus D into coherent chunks C 1 ; C 2 ; : : : ; C n , where each chunk provides a self-contained unit of information. For each chunk C i , an LLM G generates multiple hypothetical prompts Q i = q i 1 ; q i 2 ; : : : ; q ik , simu- lating possible user queries that the chunk might answer. This ofine step does not introduce any additional computational",
    //         additional_data: "What is the main idea of HyPE according to the passage?",
    //         score: 0.65756434,
    //     },
    // ];

    interface StreamMessage {
        type: "chunks" | "token" | "done";
        value?: string;
        display?: boolean;
    }

    interface Chunk {
        id: string;
        doc_id: string;
        doc_seq_num: number;
        content: string;
        additional_data: string;
        score: number;
    }

    async function sendQuery(queryParam: string) {
        waiting = true;

        conversation.push({ sender: "user", data: queryParam });

        const response = await fetch(`${PUBLIC_API_BASE_URL}/chat/search?query=${encodeURIComponent(queryParam)}`);
        const stream = response.body?.getReader();

        if (!stream) {
            console.error("Failed to get response stream");
            waiting = false;
            return;
        }

        const decoder = new TextDecoder();
        let buffer = "";

        let message = "";

        conversation.push({ sender: "assistant", data: message });

        while (true) {
            const { done, value } = await stream.read();
            if (done) break;

            buffer += decoder.decode(value, { stream: true });

            const lines = buffer.split("\n");
            buffer = lines.pop() ?? "";

            for (const line of lines) {
                if (!line.trim()) continue;
                console.log(line);

                try {
                    const msg: StreamMessage = JSON.parse(line);

                    if (msg.display) {
                        switch (msg.type) {
                            case "chunks":
                                break;
                            case "token":
                                const data = conversation.at(-1);
                                if (data) {
                                    data.data += msg.value ?? "";
                                }
                                break;
                        }
                    } else {
                        switch (msg.type) {
                            case "chunks":
                                // retrievedChunks = msg.value as Chunk[];
                                break;
                            case "token":
                                // answerTokens = [...answerTokens, msg.value as string];
                                break;
                            case "done":
                                // streaming finished
                                break;
                        }
                    }
                } catch (e) {
                    console.error("Failed to parse stream message:", line, e);
                }
            }
        }

        waiting = false;
    }

    async function vote() {
        pushNotification({ title: "Vote received", body: "Thank you for rating the information provided by our system." });
    }

    const recorder = new AudioRecorder();
    let recording = $state(false);
    let loading = $state(false);
    let warning = $state(false);

    async function toggle() {
        if (recording) {
            recording = false;
            loading = true;
            await recorder.stop((text) => (activeQuery = text));
            loading = false;
            warning = true;
            await sendQuery(activeQuery);
        } else {
            await recorder.start();
            loading = false;
            recording = true;
        }
    }

    const sendQuestion = async () => {
        if (activeQuery.length === 0) return;
        sendQuery(activeQuery);
        activeQuery = "";
    };
</script>

<div class="h-[80vh] grid">
    {#if conversation.length === 0}
        <form onsubmit={sendQuestion} class="m-auto w-[80%] card rounded-3xl p-10 grid gap-5">
            <div class="rounded-lg flex flex-col items-center justify-center h-full p-10 text-center px-6">
                <h1 class="text-2xl md:text-3xl font-semibold mb-2">Ask the expert a question</h1>
                <p class="text-sm md:text-base max-w-md">Get answers on wood construction, circular design, and sustainable building practices.</p>
            </div>
            <div class="bg-white card flex gap-2 items-end p-2">
                <textarea
                    bind:value={activeQuery}
                    name=""
                    id=""
                    placeholder="Ask your question"
                    class="resize-none w-full p-5 ring-0"
                    onkeydown={(e) => {
                        if (e.key === "Enter" && !e.shiftKey) {
                            e.preventDefault();
                            sendQuestion();
                        }
                    }}></textarea>
                {#if activeQuery.length > 0}
                    <button type="submit" in:fade class="transition-all bg-primary p-2 rounded-lg hover:bg-background">
                        <MaskedIcon src="/arrow.svg" class="bg-white" />
                    </button>
                {/if}
            </div>
        </form>
    {:else}
        <div class="p-10 grid grid-rows-1 overflow-hidden">
            <div class="card gap-5 p-5 grid grid-rows-[1fr_min-content] max-h-full overflow-hidden">
                <div class="w-full min-h-0 overflow-auto no-scrollbar text-sm prose max-w-none">
                    {#each conversation as msg}
                        {#if msg.data.length > 0}
                            <div class="flex h-min {msg.sender === 'user' ? 'justify-end' : 'justify-start'}">
                                <div class="max-w-[70%] rounded-lg px-3 py-1 {msg.sender === 'user' ? 'bg-primary text-white' : 'bg-white card'}">
                                    <div class="">{@html marked(msg.data)}</div>
                                </div>
                            </div>
                        {/if}
                    {/each}
                </div>
                <form onsubmit={sendQuestion} class="bg-white card flex gap-2 items-end p-2">
                    <textarea
                        autofocus
                        bind:value={activeQuery}
                        placeholder="Ask your question"
                        class="resize-none w-full p-3 ring-0"
                        onkeydown={(e) => {
                            if (e.key === "Enter" && !e.shiftKey) {
                                e.preventDefault();
                                sendQuestion();
                            }
                        }}></textarea>
                    {#if activeQuery.length > 0}
                        <button type="submit" in:fade class="transition-all bg-primary p-2 rounded-lg hover:bg-background">
                            <MaskedIcon src="/arrow.svg" class="bg-white" />
                        </button>
                    {/if}
                </form>
            </div>
        </div>
    {/if}
</div>
