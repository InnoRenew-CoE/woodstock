<script lang="ts">
    import { twMerge } from "tailwind-merge";
    import MaskedIcon from "./MaskedIcon.svelte";

    let { text = "Select files", files = $bindable(), multiple = true, class: className = "", accept = "" }: { text: string; files: FileList | undefined; multiple: boolean; class?: string; accept?: string } = $props();
</script>

<div class={twMerge("w-min text-nowrap", className)}>
    <label class="text-secondary/80 glass bg-secondary/10 border-secondary/50 flex gap-3 p-2 py-1 items-center justify-center hover:brightness-[90%] cursor-pointer">
        <div class="glass p-2 rounded-full bg-white/70">
            <MaskedIcon src="../download.svg" class="size-4 bg-secondary rotate-180" />
        </div>
        <div class="text-center px-5">{text}</div>
        <input class="hidden" type="file" bind:files {multiple} {accept} />
    </label>
</div>
