<script lang="ts">
    import Checkbox from "$lib/common/Checkbox.svelte";
    import { QuestionType, type Answer, type Question } from "$lib/types/question";

    let { question = $bindable(), proceed = $bindable(false) }: { question: Question; proceed: boolean } = $props();
    let answers = $state(question.answers);
    let text_answer = $state(question.text_answer);
    $effect(() => {
        question.answers = answers;
        question.text_answer = text_answer;

        proceed = (answers?.length ?? 0) > 0 || (text_answer?.length ?? 0) > 0;
    });
</script>

<div>
    <div class="text-lg font-semibold">{question.title}</div>
    <div>{question.text}</div>
    <div class="mt-5 flex gap-5 items-center justify-center flex-wrap">
        {#if question.type === QuestionType.Text}
            <textarea placeholder="Answer here..." class="p-5 min-h-[100px] resize-none w-full rounded border" bind:value={text_answer}></textarea>
        {:else}
            {#each question.possible_answers as answer}
                <Checkbox bind:group={answers} label={answer.value} value={answer.id} multiple={question.type === QuestionType.MultiSelect} />
            {/each}
        {/if}
    </div>
</div>
