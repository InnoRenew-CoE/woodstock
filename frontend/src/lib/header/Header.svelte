<script>
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { slide } from "svelte/transition";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(true);

    const paths = [
        { text: "Home", url: "/", icon: "/home.svg" },
        // { text: "About", url: "/about", icon: "/heart.svg" },
        { text: "Chat", url: "/search", icon: "/search.svg" },
        { text: "Login", url: "/login", icon: "/home.svg" },
    ];
</script>

<svelte:window bind:innerWidth={width} />
{#if !$page.url.pathname.match(/app|chat/gi)}
    <div class="pt-10 sm:p-5 flex gap-3 items-center justify-center relative uppercase text-sm">
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
                <div out:slide={{ duration: 100 }} class="transition-all absolute left-10 top-[110%] rounded-3xl right-10 z-10 bg-black p-10 sm:left-0 sm:right-0 sm:top-0 sm:relative sm:glass sm:p-1.5 sm:rounded-full sm:bg-white/30 sm:hover:px-5">
                    <div class="grid sm:flex gap-3 sm:gap-5" onclick={() => (isVisible = false)}>
                        {#each paths as path}
                            {@const isSelected = $page.url.pathname === path.url}
                            <a
                                class="cursor-pointer flex items-center gap-5 sm:opacity-50 p-1.5 glass rounded-full overflow-hidden px-8 border-white/20 {isSelected
                                    ? '!opacity-100 text-white bg-secondary/70 shadow-secondary/50 shadow-lg border-secondary sm:border-secondary'
                                    : 'bg-black sm:bg-white hover:!opacity-100 hover:text-white hover:bg-secondary/60 hover:shadow-secondary/50 hover:shadow-lg hover:border-secondary sm:text-black'} "
                                href={path.url}
                            >
                                <MaskedIcon src={path.icon} class={isSmall ? "bg-white" : "hidden"} />{path.text}
                            </a>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    </div>
{/if}
