<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Spacer from "$lib/common/Spacer.svelte";
    import { fetchQuestions, filesStore, questionsStore, submitAnswers } from "$lib/stores/questions";
    import { fade, slide } from "svelte/transition";
    import QuestionComponent from "./QuestionComponent.svelte";
    import type { Answer, FileAnswer, Question } from "$lib/types/question";
    import FileUpload from "./FileUpload.svelte";
    import Submission from "./Submission.svelte";
    import { onMount } from "svelte";

    onMount(async () => {
        await fetchQuestions();
    });

    let windowSize = $state(0);
    let proceed = $state(false);
    let lastStep = $derived(1 + $questionsStore.length * ($filesStore?.length ?? 1));
    let currentStep = $state(0);
    let fileAnswers: FileAnswer[] = $derived(
        Array.from($filesStore ?? []).flatMap((file) => {
            const answers: Answer[] = $questionsStore.map((q) => ({
                question_id: q.id,
                selection: [],
            }));
            return {
                file: file.name,
                answers: answers,
            };
        }),
    );

    let file: FileAnswer | undefined = $derived(fileAnswers[Math.floor((currentStep - 1) / $questionsStore.length)]);
    let answer: Answer | undefined = $derived(file?.answers[(currentStep - 1) % $questionsStore.length]);

    function step(forwards: boolean) {
        if ($filesStore) {
            currentStep = Math.min(Math.max(currentStep + (forwards ? 1 : -1), 0), 1 + $questionsStore.length * $filesStore.length);
            proceed = false;
        }
    }

    function submit() {
        const files = $filesStore;
        if (files && files.length > 0) {
            submitAnswers(files, fileAnswers);
        }
    }
</script>

<svelte:window bind:innerWidth={windowSize} />
{#if windowSize > 640}
    <div class="grid gap-5 grid-cols-[minmax(min-content,300px)_auto] h-full">
        <div class="select-none bg-dark-background border p-3 rounded-lg">
            <p class="text-xs opacity-40 uppercase pb-2">Step {currentStep} / {lastStep}</p>
            <ul class="p-5">
                <li class="flex items-center gap-3 {currentStep >= 0 ? 'font-bold' : ''}">
                    <MaskedIcon src="../{currentStep >= 1 ? 'checkmark.svg' : 'archive.svg'}" class="size-4 bg-secondary" />
                    File Upload
                </li>
                <div class="font-nunito opacity-30 pt-2 text-xs uppercase">Files</div>
                {#each Array.from($filesStore ?? []) as file, i}
                    {@const isVisible = currentStep >= i * $questionsStore.length + 1 && currentStep < (i + 1) * $questionsStore.length + 1}
                    {@const isDone = currentStep >= (i + 1) * $questionsStore.length + 1}
                    <div class="pl-3 flex items-center gap-2 {isDone ? 'italic text-secondary' : ''}">
                        <MaskedIcon src="../{isVisible ? 'chevron-down.svg' : isDone ? 'checkmark.svg' : 'circle.svg'}" class="w-2.5 h-2.5 bg-secondary" />
                        {file.name}
                    </div>
                    {#each $questionsStore as question, j}
                        {@const isNow = currentStep == 1 + j + i * $questionsStore.length}
                        {@const isDone = currentStep > 1 + j + i * $questionsStore.length}
                        {#if isVisible}
                            <div in:slide out:slide>
                                <li class="pl-5 {isDone || isNow ? '' : 'opacity-30'}">
                                    <div class="flex gap-3 items-center {isNow ? 'font-bold' : ''}">
                                        <MaskedIcon src="../{isDone ? 'checkmark.svg' : isNow ? 'chevron-right.svg' : 'circle.svg'}" class="w-3 h-3 bg-secondary" />
                                        {question.title}
                                    </div>
                                </li>
                            </div>
                        {/if}
                    {/each}
                {/each}
                <li class={currentStep === lastStep ? "font-bold" : ""}>Submission</li>
            </ul>
        </div>
        <div class="bg-dark-background border rounded-lg p-5 grid grid-rows-[auto_min-content]">
            {#if currentStep === 0}
                <FileUpload bind:proceed />
            {:else if currentStep === lastStep}
                {submit()}
                {JSON.stringify(fileAnswers)}
            {:else}
                {@const question = $questionsStore.filter((x) => x.id === answer?.question_id)[0]}
                {#key currentStep}
                    <div in:fade>
                        <div class="text-center">
                            <div class="font-bold">{file?.file}</div>
                            <div>
                                <span class="text-accent">{((currentStep - 1) % $questionsStore.length) + 1}</span>
                                |
                                <span class="opacity-30">{$questionsStore.length}</span>
                            </div>
                        </div>
                        <QuestionComponent bind:proceed {question} {answer} />
                    </div>
                {/key}
            {/if}
            <div class="flex justify-between gap-5">
                {#if currentStep >= 1}
                    <button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100" onclick={() => step(false)}>Back</button>
                {/if}
                <Spacer />
                {#if currentStep < lastStep}
                    {#if proceed}
                        <button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100 disabled:bg-gray-400" onclick={() => step(true)}>Next</button>
                    {:else}
                        <span class="text-secondary font-nunito flex items-center gap-2">
                            <MaskedIcon src="../chevron-right.svg" class="size-3 bg-secondary animate-pulse" />
                            Required
                        </span>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
{:else}
    <p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>
{/if}
