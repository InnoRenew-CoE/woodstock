<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    let { type = "text", placeholder, class: className = "", value = $bindable(), icon = undefined, required = false } = $props();
</script>

<div class="flex items-center bg-white hover:border-secondary border rounded-md group font-sans focus-within:border-secondary">
    {#if icon}
        <MaskedIcon src={icon} class="size-3 bg-primary ml-3 opacity-30 group-focus-within:opacity-100 group-hover:opacity-100 group-hover:bg-secondary group-focus-within:bg-secondary" />
    {/if}
    <input {required} {type} {placeholder} class="w-full px-2 py-2 outline-none transition-all ease-in-out rounded-md placeholder:text-primary/40 bg-transparent group-hover:placeholder:text-secondary {className}" bind:value />
</div>

<style>
    .input {
        width: 100%;
        height: 45px;
        line-height: 30px;
        padding: 0 5rem;
        padding-left: 3rem;
        border: 2px solid transparent;
        border-radius: 10px;
        outline: none;
        color: #0d0c22;
        transition: 0.5s ease;
    }

    .input::placeholder {
        color: #94a3b8;
    }

    .input:focus,
    .input:hover {
        outline: none;
        border-color: rgba(129, 140, 248);
        background-color: #fff;
        box-shadow: 0 0 0 5px rgb(129 140 248 / 30%);
    }
</style>
