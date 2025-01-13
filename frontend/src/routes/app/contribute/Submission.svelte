<script lang="ts">
    import type { Question } from "$lib/types/question";

    let { questions }: { questions: Question[] } = $props();
</script>

<div>
    <div class="text-lg font-semibold">Submission</div>
    <div>Thank you for answering our questions. Please submit your responses below.</div>

    <div class="flex items-center justify-center">
        <button class="my-5 px-10 py-2 text-success bg-success/5 border border-success rounded hover:brightness-50">Submit</button>
    </div>
    <pre>
        {JSON.stringify(
            Object.groupBy(questions, (a) => a.file_id!),
            null,
            1,
        )}
    </pre>
</div>
