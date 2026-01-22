<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);

    const paths = [
        { text: "Home", url: "/", icon: "/home.svg" },
        { text: "About (todo)", path: "/about", icon: "/heart.svg" },
        { text: "Contact (todo)", path: "/contact", icon: "/contact.svg" },
    ];
</script>

<svelte:window bind:innerWidth={width} />
{#if !$page.url.pathname.includes("/app")}
    <div class="pt-10 sm:p-5 flex gap-3 items-center justify-center relative">
        <div class="p-2 rounded-full glass bg-white/80">
            <img src="/woodstock.svg" class="size-7 rounded-full" />
        </div>
        <div class="rounded-full text-white p-3 {isSmall ? 'p-4 bg-black' : ''} flex gap-5 items-center justify-between">
            {#if isSmall}
                <button onclick={() => (isVisible = !isVisible)}>
                    <MaskedIcon src="../menu.svg" class="w-5 bg-white" />
                </button>
            {/if}
            {#if isVisible || !isSmall}
                <div out:slide={{ duration: 100 }} class="transition-all absolute left-10 top-[95%] rounded-3xl right-10 z-10 bg-black p-5 sm:left-0 sm:right-0 sm:top-0 sm:relative sm:glass sm:p-1.5 sm:rounded-full sm:bg-white/60 sm:hover:px-5">
                    <div class="grid sm:flex gap-3 sm:gap-5" onclick={() => (isVisible = false)}>
                        {#each paths as path}
                            {@const isSelected = window?.location?.pathname === path.url}
                            <a class="flex items-center gap-5 opacity-20 sm:opacity-30 p-1.5 glass rounded-full overflow-hidden px-8 {isSelected ? '!opacity-100 text-white bg-secondary/60 shadow-secondary/50 shadow-lg border-secondary' : 'bg-white/5 hover:bg-white sm:text-black'} " href="/">
                                <MaskedIcon src={path.icon} class={isSmall ? "bg-white" : "hidden"} />{path.text}
                            </a>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    </div>
{/if}
