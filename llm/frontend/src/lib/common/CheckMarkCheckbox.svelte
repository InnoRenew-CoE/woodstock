<script lang="ts">
    import MaskedIcon from "./MaskedIcon.svelte";

    let { class: className = "", label, checked = $bindable() } = $props();
</script>

<label class="select-none cursor-pointer rounded flex items-center gap-3">
    <input class="hidden" type="checkbox" bind:checked />
    <MaskedIcon src=" {checked ? './checkmark-selected-box.svg' : './checkmark-box.svg'}" class="size-4 {checked ? 'bg-secondary' : 'bg-black'}" />
    {label}
</label>
