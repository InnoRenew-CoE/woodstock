<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { filesStore } from "$lib/stores/questions";
    import type { Question } from "$lib/types/question";

    let { step = $bindable(), files = $bindable() }: { step: number; files: FileList | undefined } = $props();

    function reset() {
        step = 0;
        files = undefined;
    }
</script>

<div class="flex justify-center p-5">
    <div>
        <img src="../tree_of_knowledge.svg" alt="Tree of knowledge" class="size-36 my-5 mx-auto rounded-full bg-secondary/5 p-5 shadow-lg shadow-secondary/30 border border-secondary" />
        <div class="text-lg font-bold">Submission</div>
        <div>Thank you for answering our questions. Your files and answers have been safely stored.</div>
        <button class="my-3 flex items-center gap-2 justify-center w-full py-5" onclick={reset}>
            <MaskedIcon src="../chevron-right.svg" class="size-3 bg-secondary" />
            Return to file upload <b class="text-primary">here</b> or wait 15s for auto-redirection.
        </button>
    </div>
</div>
