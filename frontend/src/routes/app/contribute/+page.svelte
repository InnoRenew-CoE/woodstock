<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Spacer from "$lib/common/Spacer.svelte";
    import { filesStore, questionsStore } from "$lib/stores/questions";
    import { fade, slide } from "svelte/transition";
    import QuestionComponent from "./QuestionComponent.svelte";
    import type { Question } from "$lib/types/question";
    import FileUpload from "./FileUpload.svelte";

    let windowSize = $state(0);
    let proceed = $state(false);
    let currentStep = $state(0);
    let lastStep = $derived(1 + $questionsStore.length * ($filesStore?.length ?? 1));
    let questions: Question[] = $derived(
        Array.from($filesStore ?? []).flatMap((file) => {
            return $questionsStore.map((q) => ({
                file_id: file.name,
                ...q,
            }));
        }),
    );
    let question: Question | undefined = $derived(questions[currentStep - 1]);

    function step(forwards: boolean) {
        if ($filesStore) {
            currentStep = Math.min(Math.max(currentStep + (forwards ? 1 : -1), 0), 1 + $questionsStore.length * $filesStore.length);
            proceed = false;
        }
    }

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
    <div class="grid gap-5 grid-cols-[minmax(min-content,300px)_auto] h-full">
        <div class="select-none bg-dark-background border p-3 rounded-lg">
            <p class="text-xs opacity-40 uppercase pb-2">Step {currentStep + 1} / {lastStep + 1}</p>
            <ul class="p-5">
                <li class="flex items-center gap-3 {currentStep >= 0 ? 'font-bold' : ''}">
                    <MaskedIcon src="../{currentStep >= 0 ? 'checkmark.svg' : 'circle.svg'}" class="w-2 h-2 bg-secondary" />
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
                Submissssssion
                {console.log(Object.groupBy(questions, (a) => a.file_id!))}
            {:else}
                {#key currentStep}
                    <div in:fade>
                        <div class="text-center">
                            <div class="font-bold">{question?.file_id}</div>
                            <div>
                                <span class="text-accent">{((currentStep - 1) % $questionsStore.length) + 1}</span>
                                |
                                <span class="opacity-30">{$questionsStore.length}</span>
                            </div>
                        </div>
                        <QuestionComponent bind:proceed {question} />
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
                            Kindly complete the question to continue.
                        </span>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
{:else}
    <p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>
{/if}
