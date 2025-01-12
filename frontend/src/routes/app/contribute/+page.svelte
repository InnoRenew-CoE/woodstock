<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Spacer from "$lib/common/Spacer.svelte";
    import { findParentQuestionIndex, questionsStore } from "$lib/stores/questions";
    import { fade, slide } from "svelte/transition";

    let windowSize = $state(0);
    let currentStep = $state(0);

    $effect(() => {
        currentStep = Math.min(Math.max(currentStep, 0), $questionsStore.length - 1);
    });

    const addQuestions = (index: number) => {
        $questionsStore.splice(index, 0, { level: 1, title: "test.pdf", text: "Please answer the following questions about the file test.pdf" }, { level: 2, title: "Document type", possibleAnswers: ["A", "B", "C"] }, { level: 2, title: "QA" }, { level: 2, title: "QB" });
        $questionsStore = [...$questionsStore];
    };
</script>

<svelte:window bind:innerWidth={windowSize} />
{#if windowSize > 640}
    <div class="grid gap-5 grid-cols-[minmax(250px,10%)_auto] h-full">
        <div class="bg-dark-background border p-3 rounded-lg">
            <p class="text-xs opacity-40 uppercase">Steps</p>
            <div class="flex justify-between gap-5">
                <button class="p-1 text-xs flex-1 bg-light-background border rounded-lg" on:click={() => currentStep--}>-1</button>
                <button class="p-1 text-xs flex-1 bg-light-background border rounded-lg" on:click={() => addQuestions(1)}>Add</button>
                <button class="p-1 text-xs flex-1 bg-light-background border rounded-lg" on:click={() => currentStep++}>+1</button>
            </div>
            <ul class="p-5">
                {#each $questionsStore as question, i}
                    {@const parentIndex = findParentQuestionIndex(question) ?? -1}
                    {@const isSelected = currentStep >= i}
                    {#if question.level < 2 || (currentStep >= parentIndex.min && currentStep < parentIndex.max)}
                        <li in:slide out:slide style="padding-left: {question.level * 1.25}rem;" class="transition-all {currentStep >= i ? 'font-semibold' : 'opacity-30'}">
                            <div class="flex justify-between items-center">
                                <div class="grid grid-cols-[1rem_auto] gap-1 items-center">
                                    <MaskedIcon src={currentStep < parentIndex.min && currentStep > parentIndex.max ? "../circle-filled.svg" : "../circle.svg"} class="w-2.5 bg-accent" />
                                    {question.title}
                                </div>
                            </div>
                        </li>
                    {/if}
                {/each}
            </ul>
        </div>
        <div class="bg-dark-background border rounded-lg p-5 font-bold text-lg">
            {#if currentStep === 0}
                FileUpload
            {:else if currentStep === $questionsStore.length - 1}
                Submissssssion
            {:else}
                {$questionsStore[currentStep]?.text ?? JSON.stringify($questionsStore[currentStep]?.possibleAnswers)}
            {/if}
        </div>
    </div>
{:else}
    <p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>
{/if}
