<script lang="ts">
    import Checkbox from "$lib/common/Checkbox.svelte";
    import { QuestionType, type Answer, type Question } from "$lib/types/question";

    let { question, answer = $bindable(), proceed = $bindable(false) }: { question: Question; answer: Answer; proceed: boolean } = $props();
    let selection = $state(answer.selection);
    let text_answer = $state(answer.text);
    $effect(() => {
        answer.selection = selection;
        answer.text = text_answer;
        proceed = selection.length > 0 || (text_answer.length ?? 0) > 0;
    });
</script>

{#if answer}
    <div>
        <div class="text-lg font-semibold">{question.title}</div>
        <div>{question.text}</div>
        <div class="mt-5 flex gap-5 items-center justify-center flex-wrap">
            {#if question.question_type === QuestionType.Text}
                <textarea placeholder="Answer here..." class="p-5 min-h-[100px] resize-none w-full rounded border" bind:value={text_answer}></textarea>
            {:else}
                {#each question.possible_answers as possible_answer}
                    <Checkbox bind:group={selection} label={possible_answer.value} value={possible_answer.id} multiple={question.question_type === QuestionType.MultiSelect} />
                {/each}
            {/if}
        </div>
    </div>
{:else}
    {(proceed = true)}
{/if}
