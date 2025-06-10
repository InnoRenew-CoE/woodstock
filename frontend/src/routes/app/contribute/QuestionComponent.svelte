<script lang="ts">
    import Checkbox from "$lib/common/Checkbox.svelte";
    import SuggestiveInput from "$lib/common/SuggestiveInput.svelte";
    import { QuestionType, type Answer, type Question } from "$lib/types/question";
    import { tagsStore } from "$lib/stores/questions";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    let { question, answer = $bindable(), proceed = $bindable(false) }: { question: Question; answer: Answer; proceed: boolean } = $props();
    let selection = $state(answer.selection);
    let text_answer = $state(answer.text);
    let tags = $state(answer.tags);

    let multipleTextAnswers = $state([""]);
    $effect(() => {
        answer.selection = selection;
        answer.text = text_answer;
        answer.tags = tags;
        if (question.question_type === QuestionType.MultiText) {
            const options = multipleTextAnswers.filter((s) => s.trim().length !== 0);
            if (options.length > 0) {
                answer.text = JSON.stringify(options);
            }
        }
        proceed = answer.selection.length > 0 || (answer.text?.length ?? 0) > 0 || answer.tags.length > 0;
    });
</script>

{#if answer}
    <div>
        <div class="glass p-8">
            <div class="text-lg font-semibold">{question.title}</div>
            <div>{question.text}</div>
        </div>
        <div class="mt-5 flex gap-5 items-center justify-center flex-wrap">
            {#if question.question_type === QuestionType.Text}
                <textarea placeholder="Answer here..." class="glass p-5 min-h-[100px] resize-none w-full" bind:value={text_answer}></textarea>
            {:else if question.question_type === QuestionType.MultiText}
                <div class="grid w-full gap-3 py-5">
                    {#each multipleTextAnswers as _, i}
                        <div class="flex items-center gap-3">
                            <input
                                placeholder="Type here..."
                                class="glass p-5 py-2 resize-none w-full"
                                value={multipleTextAnswers[i]}
                                onkeyup={(e) => {
                                    const value = e.currentTarget.value ?? "";
                                    multipleTextAnswers[i] = value;
                                    if (value.trim().length > 0 && multipleTextAnswers.find((a) => a.length === 0) === undefined) {
                                        multipleTextAnswers = [...multipleTextAnswers, ""];
                                    }
                                }}
                            />
                            <div
                                class="glass p-2 hover:bg-red-50 cursor-pointer"
                                onclick={() => {
                                    multipleTextAnswers = multipleTextAnswers.toSpliced(i, 1);
                                    multipleTextAnswers = [...multipleTextAnswers];
                                }}
                            >
                                <MaskedIcon src="/x-close.svg" class="size-4 bg-red-600" />
                            </div>
                        </div>
                    {/each}
                </div>
            {:else if question.question_type === QuestionType.Tags}
                <SuggestiveInput bind:selected={tags} options={$tagsStore} />
            {:else}
                <div class="grid sm:flex gap-3 w-full items-stretch flex-wrap">
                    {#each question.possible_answers.toSorted() as possible_answer}
                        <Checkbox bind:group={selection} label={possible_answer.value} value={possible_answer.id} multiple={question.question_type === QuestionType.MultiSelect} />
                    {/each}
                </div>
            {/if}
        </div>
    </div>
{:else}
    {(proceed = true)}
{/if}
