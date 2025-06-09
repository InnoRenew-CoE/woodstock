<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);
</script>

<svelte:window bind:innerWidth={width} />
{#if !$page.url.pathname.includes("/app")}
    <div class="p-3 flex items-center justify-center relative">
        <div class="rounded-full bg-black text-white px-10 py-1 {isSmall ? 'py-2' : ''} flex gap-5 items-center justify-between border-b border-b-secondary">
            <img src="/woodstock.svg" class="size-10 rounded-full" />
            {#if isSmall}
                <button onclick={() => (isVisible = !isVisible)}>
                    <MaskedIcon src="../menu.svg" class="w-5 bg-white" />
                </button>
            {/if}
            {#if isVisible || !isSmall}
                <div
                    in:slide
                    out:slide={{ duration: 100 }}
                    class="z-10 absolute bottom-0 translate-y-full bg-black rounded-2xl text-white right-10 left-10 p-3
                    sm:relative sm:flex sm:translate-y-0 sm:left-0 sm:right-0 sm:p-0"
                >
                    <ul class="pl-8 py-1 grid sm:flex gap-3 sm:gap-10" onclick={() => (isVisible = false)}>
                        <li class="hover:text-secondary">
                            <a class="flex items-center gap-3" href="/">
                                <MaskedIcon src="../home.svg" class={isSmall ? "bg-white" : "hidden"} />Home
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center gap-3" href="/">
                                <MaskedIcon src="../book-open.svg" class={isSmall ? "bg-white" : "hidden"} />About
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center gap-3" href="/">
                                <MaskedIcon src="../contact.svg" class={isSmall ? "bg-white" : "hidden"} />Contact
                            </a>
                        </li>
                    </ul>
                </div>
            {/if}
        </div>
    </div>
{/if}
