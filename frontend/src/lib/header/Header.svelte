<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);

    const paths = [
        { text: "Home", url: "/", icon: "/home.svg" },
        { text: "About", path: "/about", icon: "/heart.svg" },
        { text: "Contact", path: "/contact", icon: "/contact.svg" },
    ];
</script>

<svelte:window bind:innerWidth={width} />
{#if !$page.url.pathname.includes("/app")}
    <div class="p-2 sm:p-5 flex gap-3 items-center justify-center relative">
        <div class="p-2 rounded-full glass bg-white/80">
            <img src="/woodstock.svg" class="size-7 rounded-full" />
        </div>
        <div class="rounded-full text-white {isSmall ? 'p-4 bg-black' : ''} flex gap-5 items-center justify-between">
            {#if isSmall}
                <button onclick={() => (isVisible = !isVisible)}>
                    <MaskedIcon src="../menu.svg" class="w-5 bg-white" />
                </button>
            {/if}
            {#if isVisible || !isSmall}
                <div
                    in:slide
                    out:slide={{ duration: 100 }}
                    class="z-10 absolute bottom-0 translate-y-full bg-black rounded-2xl text-white right-10 left-10 p-4
                    sm:relative sm:rounded-full sm:py-1.5 sm:px-2 sm:flex sm:translate-y-0 sm:left-0 sm:right-0"
                >
                    <ul class="grid sm:flex gap-3 sm:gap-2" onclick={() => (isVisible = false)}>
                        {#each paths as path}
                            {@const isSelected = window?.location?.pathname === path.url}
                            <li>
                                <a class="cursor-pointer flex items-center gap-3 {isSelected ? 'bg-white text-black' : ''} px-8 py-1.5 rounded-full" href={path.url}>
                                    <MaskedIcon src={path.icon} class={isSmall ? (isSelected ? "bg-black" : "bg-white") : "hidden"} />{path.text}
                                </a>
                            </li>
                        {/each}
                    </ul>
                </div>
            {/if}
        </div>
    </div>
{/if}
