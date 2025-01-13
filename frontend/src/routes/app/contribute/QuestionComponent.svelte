<script lang="ts">
    import { QuestionType, type Answer, type Question } from "$lib/types/question";
    import { onDestroy } from "svelte";

    let { question, answers = $bindable() }: { question: Question; answers: Answer[] } = $props();

    /*
    Text => string
    Selection => array
    */
</script>

{#if question.type === QuestionType.Text}
    <textarea></textarea>
{:else}
    {#each question.possible_answers as answer}
        <input id={answer.id.toString()} type="checkbox" value={answer.id} />
        <label for={answer.id.toString()}>{answer.value}</label>
    {/each}
{/if}
