<script lang="ts">
    import { slide } from "svelte/transition";
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    let { children } = $props();
    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);

    const paths: { link: string; text: string }[] = [
        { link: "/app", text: "Home" },
        { link: "/app/search", text: "Search" },
        { link: "/app/contribute", text: "Contribute" },
        { link: "/app/notifications", text: "Notifications" },
    ];
    let notificationCount = $state(0);
</script>

<svelte:window bind:innerWidth={width} />

<div class="p-2 sm:p-5 h-full bg-dark-background">
    <div class="flex flex-col items-end sm:items-center h-full gap-5">
        <div class="sm:relative sm:rounded-2xl sm:border sm:bg-primary text-white sm:flex sm:p-3">
            <div class="bg-black rounded-xl p-2 sm:bg-transparent sm:border-transparent sm:rounded-none sm:p-0 flex gap-5 items-center justify-end w-full">
                {#if isSmall}
                    <button onclick={() => (isVisible = !isVisible)}>
                        <MaskedIcon src="../menu.svg" class="w-10 p-3 bg-white" />
                    </button>
                {/if}
                {#if isVisible || !isSmall}
                    <div in:slide out:slide={{ duration: 100 }} class="z-10 select-none top-16 left-0 right-0 bg-black absolute text-white p-5 sm:relative sm:top-0 sm:translate-y-0 sm:p-0">
                        <ul class="grid sm:flex gap-3 sm:gap-3 font-nunito" onclick={() => (isVisible = false)}>
                            {#each paths as { link, text }}
                                {@const isSelected = link == $page.url.pathname}
                                <li>
                                    <a class="box-border border border-transparent px-5 py-1 rounded-lg transition-all sm:hover:bg-light-background/30 sm:hover:border-white/10 {isSelected ? 'bg-light-background/20 border border-white/10' : ''}" href={link}>
                                        {text}
                                        {#if text === "Notifications"}
                                            <div class="m-1 inline-block rounded-full text-xs bg-secondary/90 border-secondary border p-0.5 animate-pulse"></div>
                                        {/if}
                                    </a>
                                </li>
                            {/each}
                        </ul>
                    </div>
                {/if}
            </div>
        </div>
        <div class="rounded-xl p-10 border bg-light-background w-full flex-1">{@render children()}</div>
    </div>
</div>
