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
    import { tagsStore } from "$lib/stores/questions";
    import SuggestiveInput from "$lib/common/SuggestiveInput.svelte";

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
                tags: [],
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

    $effect(() => {
        if (currentStep === lastStep) {
            const files = $filesStore;
            if (files && files.length > 0) {
                submitAnswers(files, fileAnswers);
            }
            setTimeout(() => {
                $filesStore = undefined;
                currentStep = 0;
            }, 15_000);
        }
    });
</script>

<svelte:window bind:innerWidth={windowSize} />

{#if windowSize > 640}
    <div class="glass p-5 grid gap-5 grid-cols-[minmax(min-content,300px)_auto] h-full">
        <div class="select-none glass px-4 py-2">
            <!-- <p class="text-xs opacity-40 uppercase pb-2">Step {currentStep} / {lastStep}</p> -->
            <ul class="p-5 space-y-3">
                <li class="flex items-center gap-3 {currentStep === 0 ? 'font-semibold' : ''}">
                    <MaskedIcon src="../{currentStep >= 1 ? 'checkmark.svg' : 'circle.svg'}" class="size-3 bg-primary" />
                    File Selection
                </li>
                {#if ($filesStore?.length ?? 0) > 0}
                    <div class="font-nunito opacity-30 pt-2 text-xs uppercase">Files</div>
                {/if}
                {#each Array.from($filesStore ?? []) as file, i}
                    {@const isVisible = currentStep >= i * $questionsStore.length + 1 && currentStep < (i + 1) * $questionsStore.length + 1}
                    {@const isDone = currentStep >= (i + 1) * $questionsStore.length + 1}
                    {@const backgroundColor = isDone ? "bg-lime-400" : "bg-primary"}
                    <div class="glass group relative px-3 py-1 shadow-sm border {isDone ? 'border-lime-400 bg-lime-400/10' : ''}">
                        <div class="flex items-center gap-2 {isDone ? 'text-lime-500' : ''} ">
                            <div class="text-white glass bg-secondary z-20 absolute top-0 left-0 pr-30 text-nowrap py-1 px-5 cursor-pointer hidden group-hover:block">
                                {file.name}
                            </div>
                            <MaskedIcon src="../{isVisible ? 'chevron-down.svg' : isDone ? 'checkmark.svg' : 'circle.svg'}" class="size-3 {backgroundColor}" />
                            {#if file.name.length > 10}
                                {file.name.slice(0, 3)} ... {file.name.slice(-5)}
                            {:else}
                                {file.name}
                            {/if}
                        </div>
                        {#each $questionsStore as question, j}
                            {@const isNow = currentStep == 1 + j + i * $questionsStore.length}
                            {@const isDone = currentStep > 1 + j + i * $questionsStore.length}
                            {#if isVisible}
                                <div in:slide out:slide>
                                    <li class="pt-2 pl-5 {isDone || isNow ? '' : 'opacity-30'}">
                                        <div class="flex gap-3 items-center {isNow ? 'text-secondary' : ''}">
                                            <MaskedIcon src="../{isDone ? 'checkmark.svg' : isNow ? 'chevron-right.svg' : 'circle.svg'}" class="w-3 h-3 bg-secondary" />
                                            {question.title}
                                        </div>
                                    </li>
                                </div>
                            {/if}
                        {/each}
                    </div>
                {/each}
                <li class="flex items-center gap-3 {currentStep === lastStep ? 'font-bold' : ''}">
                    <!-- <MaskedIcon src="../{currentStep === lastStep ? 'checkmark.svg' : 'circle.svg'}" class="z-0 size-3 bg-secondary" /> -->
                    Submission
                </li>
            </ul>
        </div>
        <div class="glass p-10 grid grid-rows-[auto_min-content]">
            {#if currentStep === 0}
                <FileUpload bind:proceed />
            {:else if currentStep === lastStep}
                <Submission bind:step={currentStep} bind:files={$filesStore} />
            {:else if answer}
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
            <div class="flex justify-between gap-5 py-5">
                {#if currentStep >= 1 && currentStep !== lastStep}
                    <button class="transition-all py-1 px-3 rounded bg-black text-white hover:bg-amber-500 cursor-pointer" onclick={() => step(false)}>Back</button>
                {/if}
                <Spacer />
                {#if currentStep < lastStep}
                    {#if proceed}
                        <button class="transition-all py-1 px-3 rounded bg-black text-white hover:bg-accent cursor-pointer disabled:bg-gray-400" onclick={() => step(true)}>Next</button>
                    {:else}
                        <span class="font-nunito flex items-center gap-2">
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
