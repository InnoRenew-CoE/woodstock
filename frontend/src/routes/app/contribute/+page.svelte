<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Spacer from "$lib/common/Spacer.svelte";
    import { filesStore, questionsStore } from "$lib/stores/questions";
    import { fade, slide } from "svelte/transition";
    import QuestionComponent from "./QuestionComponent.svelte";
    import type { Question } from "$lib/types/question";

    let windowSize = $state(0);
    let currentStep = $state(1);
    let lastStep = $derived(1 + $questionsStore.length * $filesStore.length);
    let currentQuestion: Question | undefined = $derived($questionsStore[(currentStep - 1) % $questionsStore.length]);
    let currentFile: string | undefined = $derived($filesStore[Math.ceil(currentStep / $questionsStore.length) - 1]);

    function step(forwards: boolean) {
        if (currentStep !== 0 && currentStep !== lastStep) {
            console.log(`Previous question: ${currentQuestion?.title} @ \t\t${currentFile}`);
        }
        currentStep = Math.min(Math.max(currentStep + (forwards ? 1 : -1), 0), 1 + $questionsStore.length * $filesStore.length);
    }
    const addQuestions = (index: number) => {};

    // let direction = true;
    // setInterval(() => {
    //     if (currentStep === 0) {
    //         direction = true;
    //     } else if (currentStep === 1 + $questionsStore.length * $filesStore.length) {
    //         direction = false;
    //     }
    //     currentStep += direction ? 1 : -1;
    // }, 250);
</script>

<svelte:window bind:innerWidth={windowSize} />
{#if windowSize > 640}
    <div class="grid gap-5 grid-cols-[minmax(250px,10%)_auto] h-full">
        <div class="bg-dark-background border p-3 rounded-lg">
            <p class="text-xs opacity-40 uppercase pb-2">Step {currentStep + 1} / {lastStep + 1}</p>
            <div class="flex justify-between gap-5">
                <button class="p-1 text-xs flex-1 bg-light-background border rounded-lg" onclick={() => step(false)}>-1</button>
                <button class="p-1 text-xs flex-1 bg-light-background border rounded-lg" onclick={() => step(true)}>+1</button>
            </div>
            <ul class="p-5">
                <li class="flex items-center gap-3 {currentStep >= 0 ? 'font-bold' : ''}">
                    <MaskedIcon src="../{currentStep >= 0 ? 'checkmark.svg' : 'circle.svg'}" class="w-2 h-2 bg-secondary" />
                    File Upload
                </li>
                <div class="font-nunito opacity-30 pt-2 text-xs uppercase">Files</div>
                {#each $filesStore as file, i}
                    {@const isVisible = currentStep >= i * $questionsStore.length + 1 && currentStep < (i + 1) * $questionsStore.length + 1}
                    {@const isDone = currentStep >= (i + 1) * $questionsStore.length + 1}
                    <div class="pl-3 flex items-center gap-2 {isDone ? 'italic text-secondary' : ''}">
                        <MaskedIcon src="../{isVisible ? 'chevron-down.svg' : isDone ? 'checkmark.svg' : 'circle.svg'}" class="w-2.5 h-2.5 bg-secondary" />
                        {file}
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
        <div class="bg-dark-background border rounded-lg p-5 font-bold text-lg">
            {#if currentStep === 0}
                FileUpload
            {:else if currentStep === $filesStore.length * $questionsStore.length + 1}
                Submissssssion
            {:else}
                <div>File: {Math.ceil(currentStep / $questionsStore.length)}</div>
                <div>Name: {$filesStore[Math.ceil(currentStep / $questionsStore.length) - 1]}</div>
                <QuestionComponent question={currentQuestion} />
            {/if}
        </div>
    </div>
{:else}
    <p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>
{/if}
