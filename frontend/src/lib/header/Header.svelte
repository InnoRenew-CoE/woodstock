<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);
</script>

<svelte:window bind:innerWidth={width} />
<div class="relative bg-dark-background px-5 {isSmall ? 'py-2' : ''} flex items-center justify-between">
    {#if !$page.url.pathname.includes("/app")}
        <div class="flex">
            <img src="../woodstock.svg" class="h-14 rounded-full" />
        </div>
        {#if isSmall}
            <button on:click={() => (isVisible = !isVisible)}> <img src="../menu.svg" alt="menu" class="w-5" /></button>
        {/if}
        {#if isVisible || !isSmall}
            <div
                in:slide
                out:slide={{ duration: 100 }}
                class="absolute bottom-0 translate-y-full bg-secondary text-white right-0 left-0 p-3
                sm:relative sm:flex sm:translate-y-0 sm:bg-transparent sm:text-primary"
            >
                <ul class="pl-8 py-4 grid sm:flex gap-3 sm:gap-10" on:click={() => (isVisible = false)}>
                    <li>
                        <a class="flex items-center gap-3 sm:hover:text-accent" href="/">
                            <MaskedIcon src="../home.svg" class={isSmall ? "bg-white" : "hidden"} />Home
                        </a>
                    </li>
                    <li>
                        <a class="flex items-center gap-3 sm:hover:text-accent" href="/account/login">
                            <MaskedIcon src="../book-open.svg" class={isSmall ? "bg-white" : "hidden"} />About
                        </a>
                    </li>
                    <li>
                        <a class="flex items-center gap-3 sm:hover:text-accent" href="/account/login">
                            <MaskedIcon src="../contact.svg" class={isSmall ? "bg-white" : "hidden"} />Contact
                        </a>
                    </li>
                </ul>
            </div>
        {/if}
    {/if}
</div>
