<script lang="ts">
    import { tagsStore } from "$lib/stores/questions";
    import { fade, fly, slide } from "svelte/transition";

    let { options, selected = $bindable() }: { options: string[]; selected: string[] } = $props();

    let currentText = $state("");

    async function change(e: KeyboardEvent) {
        if (e.code.match(/Comma|Enter/)) {
            e.preventDefault();
            const tag = currentText;
            currentText = "";
            add(tag);
        }
    }

    async function add(tag: string) {
        if (tag.length >= 2 && selected.find((s) => s === tag) === undefined) {
            selected = [...selected, tag];
        }
        currentText = "";
        console.log(selected);
        selected = selected;
    }

    async function remove(tag: string) {
        selected = selected.filter((s) => s !== tag);
    }
</script>

<div class="grid w-full gap-3 relative">
    <input autocomplete="off" id="tag-input" class="outline-none focus:border-secondary rounded-xl glass appearance-none px-3 py-2 w-full" placeholder="Eg: wood, glue, ..." bind:value={currentText} onkeypress={change} />
    <ul class="flex gap-3">
        {#each selected as tag}
            <li in:fly={{ x: 150 }} class="transition-all glass px-3 py-2 rounded-lg" onclick={() => remove(tag)}>
                {tag}
            </li>
        {/each}
    </ul>
    {#if currentText.length > 0}
        <div in:fade class="select-none z-10 absolute top-[110%] right-0 left-0 bg-gray-50 p-5 rounded-lg shadow border border-white">
            <div class="opacity-50 font-mono pb-3">Tag Suggestions</div>
            <ul in:fade class="max-h-[100px] flex items-center gap-3 overflow-auto list-none">
                {#each options.filter((o) => o.includes(currentText)) as option}
                    <li in:fade class="w-min rounded-sm border bg-secondary/20 border-secondary text-secondary px-3 py-1 hover:bg-accent/80 hover:border-accent cursor-pointer hover:text-white transition-all" onclick={() => add(option)}>{option}</li>
                {/each}
            </ul>
            {#if options.filter((o) => o.includes(currentText)).length === 0}
                <span class="text-amber-600">You're the first one to add this tag!</span>
            {/if}
        </div>
    {/if}
</div>
