<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Spacer from "$lib/common/Spacer.svelte";
    import { pushNotification } from "$lib/stores/notifications";
    import { marked } from "marked";
    import { SvelteMap } from "svelte/reactivity";
    import { fade } from "svelte/transition";
    import { AudioRecorder } from "./AudioRecorder";

    interface Message {
        role: string;
        content: string;
    }

    let activeQuery = $state("");
    let conversation = $state<Message[]>([]);
    let extendedDocument: string | undefined = $state(undefined);
    let files = new SvelteMap<string, Chunk[]>();
    let fileMap = new SvelteMap<string, string>(); // chunk-id, doc-id

    let fileContainer = $state();
    let waiting = $state(false);
    let hoveredId: string | undefined = $state(undefined);

    const setHoveredId = (id: string) => {
        hoveredId = id;
        const docId = fileMap.get(id);
        if (docId) {
            extendedDocument = docId;
            fileContainer?.scrollIntoView({ behavior: "smooth", block: "end" });
        }
    };

    window.setHoveredId = setHoveredId;

    interface StreamMessage {
        type: "chunks" | "token" | "done";
        value?: any;
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

        conversation.push({ role: "user", content: queryParam });
        const response = await fetch(`${PUBLIC_API_BASE_URL}/chat/search`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                query: activeQuery,
                history: conversation,
            }),
        });
        const stream = response.body?.getReader();

        if (!stream) {
            console.error("Failed to get response stream");
            waiting = false;
            return;
        }

        const decoder = new TextDecoder();
        let buffer = "";

        let message = "";

        conversation.push({ role: "assistant", content: message });

        while (true) {
            const { done, value } = await stream.read();
            if (done) break;

            buffer += decoder.decode(value, { stream: true });

            const lines = buffer.split("\n");
            buffer = lines.pop() ?? "";

            for (const line of lines) {
                if (!line.trim()) continue;

                try {
                    const msg: StreamMessage = JSON.parse(line);
                    switch (msg.type) {
                        case "chunks":
                            const chunks = msg.value as Chunk[];
                            for (const chunk of chunks) {
                                fileMap.set(chunk.id, chunk.doc_id);
                                const f = files.get(chunk.doc_id) ?? [];
                                f.push(chunk);
                                files.set(chunk.doc_id, f);
                            }
                            break;
                        case "token":
                            const data = conversation.at(-1);
                            if (data && msg.display) {
                                data.content += msg.value ?? "";
                            }
                            break;
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

    const sampleQuestions = ["Is reclaimed wood safe to use for structural purpose?", "Which materials should I use to build according to NEB values?", "Are any good design examples to build with wood?"];
</script>

<div class="h-full max-h-[80vh] grid">
    {#if conversation.length === 0}
        <form onsubmit={sendQuestion} class="m-auto w-[80%] card rounded-3xl p-10 grid gap-5">
            <div class="rounded-lg flex flex-col items-center justify-center h-full p-10 text-center px-6">
                <h1 class="text-2xl md:text-3xl font-semibold mb-2">Ask the expert a question</h1>
                <p class="text-sm md:text-base max-w-md">Get answers on wood construction, circular design, and sustainable building practices.</p>
            </div>
            <div class="grid gap-2">
                <div class="uppercase text-xs opacity-30">Sample questions (click to try)</div>
                <div class="flex gap-3 text-xs items-start">
                    {#each sampleQuestions as question}
                        <div
                            class="card p-2 bg-white hover:bg-amber-200/30 hover:text-amber-600 cursor-pointer"
                            onclick={() => {
                                sendQuery(question);
                                activeQuery = "";
                            }}
                        >
                            {question}
                        </div>
                    {/each}
                </div>
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
                {#if recording}
                    <span class="text-info font-light animate-pulse text-xs whitespace-nowrap">We're listening to your dictation...</span>
                {/if}

                <button in:fade class="transition-all bg-info p-2 rounded-lg hover:bg-background" onclick={toggle}>
                    <MaskedIcon src="/microphone.svg" class="bg-white" />
                </button>
                {#if activeQuery.length > 0}
                    <button type="submit" in:fade class="transition-all bg-primary p-2 rounded-lg hover:bg-background">
                        <MaskedIcon src="/arrow.svg" class="bg-white" />
                    </button>
                {/if}
            </div>
        </form>
    {:else}
        <div class="p-10 gap-5 grid grid-rows-1 grid-cols-[minmax(300px,2fr)_3fr] overflow-hidden">
            <div bind:this={fileContainer} id="files" class="flex-col flex gap-2 items-start h-full overflow-auto min-h-0 no-scrollbar">
                {#each files as [doc_id, chunks]}
                    {@const isExtended = extendedDocument === doc_id}
                    {@const isHovered = fileMap.get(hoveredId ?? "") === doc_id}
                    {@const firstChunk = chunks[0]}
                    <div onclick={() => (isExtended ? (extendedDocument = undefined) : (extendedDocument = doc_id))} id={doc_id} class="card w-full bg-white">
                        <div class="w-full flex items-center gap-3 rounded-lg px-3 py-2 transition-colors cursor-pointer">
                            <div class="text-sm text-gray-700">{firstChunk.additional_data}</div>
                            <Spacer />
                            <div class="font-mono text-xs rounded-lg flex items-center justify-center font-light text-secondary">
                                {firstChunk.score.toFixed(2)}
                            </div>
                            <a href="{PUBLIC_API_BASE_URL}/chat/download/{doc_id}" class="bg-primary rounded-lg p-2">
                                <MaskedIcon src="/download.svg" class="bg-white size-3" />
                            </a>
                        </div>
                        {#if isExtended}
                            <div in:fade class="overflow-auto max-h-full min-h-0 grid gap-5 p-5">
                                {#each chunks as chunk}
                                    {@const isHovered = hoveredId === chunk.id}
                                    <div id={chunk.id} class="prose text-xs max-w-none p-5 card bg-white {isHovered ? 'bg-amber-200! ' : ''}">
                                        {#await marked(chunk.content) then data}
                                            {@html data}
                                        {/await}
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
            <div id="conversation" class="card gap-5 p-5 grid grid-rows-[1fr_min-content] max-h-full overflow-hidden">
                <div class=" w-full min-h-0 overflow-auto no-scrollbar text-sm prose max-w-none">
                    {#each conversation as msg}
                        {#if msg.content.length > 0}
                            <div class="flex {msg.role === 'user' ? 'justify-end' : 'justify-start'}">
                                <div class="max-w-[70%] rounded-xl p-2 {msg.role === 'user' ? 'bg-primary text-white' : 'bg-white card'}">
                                    <div class="[&_p]:m-0!">
                                        {#await marked(msg.content) then data}
                                            {@html (() => {
                                                let i = 0;
                                                return data.replace(/\[\[(.*?)\]\]/gi, (match, id) => {
                                                    i += 1;
                                                    return `<a href="#${id}" class="p-1 card text-amber-700 hover:text-amber-900 text-xs font-mono px-1 underline underline-offset-2" onmouseover="setHoveredId('${id}')">${i}</a>`;
                                                });
                                            })()}
                                        {/await}
                                    </div>
                                </div>
                            </div>
                        {/if}
                    {/each}
                    {#if waiting}
                        <span class="loading-dots p-3">
                            <span></span><span></span><span></span>
                        </span>
                    {/if}
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
                    {#if recording}
                        <span class="text-info font-light animate-pulse text-xs whitespace-nowrap">We're listening to your dictation...</span>
                    {/if}

                    <button in:fade class="transition-all bg-info p-2 rounded-lg hover:bg-background" onclick={toggle}>
                        <MaskedIcon src="/microphone.svg" class="bg-white" />
                    </button>
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

<style>
    .loading-dots {
        display: inline-flex;
        gap: 4px;
        padding: 8px 0;
    }
    .loading-dots span {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: black;
        animation: dot-bounce 1.4s infinite ease-in-out both;
    }
    .loading-dots span:nth-child(1) {
        animation-delay: -0.32s;
    }
    .loading-dots span:nth-child(2) {
        animation-delay: -0.16s;
    }
    .loading-dots span:nth-child(3) {
        animation-delay: 0s;
    }
    @keyframes dot-bounce {
        0%,
        80%,
        100% {
            transform: scale(0);
        }
        40% {
            transform: scale(1);
        }
    }
</style>
