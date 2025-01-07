<script lang="ts">
    import { slide } from "svelte/transition";
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    let { children } = $props();
    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);
</script>

<svelte:window bind:innerWidth={width} />

<div class="p-5 h-full bg-dark-background">
    <div class="grid {isSmall ? 'pt-16' : 'grid-cols-[minmax(250px,10%)_auto]'} h-full gap-5">
        <div class="absolute top-0 left-0 right-0 rounded-b-xl px-5 sm:px-0 sm:relative sm:rounded-xl sm:border py-5 sm:bg-light-background">
            <div id="logo" class="flex items-center justify-between sm:block">
                <div class="sm:w-full items-center justify-center flex">
                    <img src="../woodstock.svg" alt="logo" class="h-14 rounded-full" />
                </div>
                {#if isSmall}
                    {$page.url.pathname}
                    <button on:click={() => (isVisible = !isVisible)}>
                        <img src="../menu.svg" alt="menu" class="w-5" />
                    </button>
                {/if}
                {#if isVisible || !isSmall}
                    <div in:slide out:slide={{ duration: 100 }} class="absolute bottom-0 translate-y-full bg-secondary text-white right-0 left-0 p-3 sm:relative sm:translate-y-0 sm:bg-transparent sm:text-primary sm:p-5">
                        <p class="font-sans uppercase text-xs opacity-20">Dashboard</p>
                        <ul class="py-4 grid gap-3 sm:gap-3" on:click={() => (isVisible = false)}>
                            <li>
                                <a class="border px-3 py-1 rounded-md bg-secondary/5 border-secondary/10 sm:text-secondary flex items-center gap-3 sm:hover:text-secondary" href="/app/home">
                                    <MaskedIcon src="../home.svg" class={isSmall ? "bg-white" : "bg-secondary"} />
                                    Home
                                </a>
                            </li>
                            <li>
                                <a class="border border-transparent px-3 py-1 rounded-md group sm:hover:pl-5 transition-all ease-in-out flex items-center gap-3 sm:hover:text-secondary" href="/app/search">
                                    <MaskedIcon src="../search.svg" class={isSmall ? "bg-white" : "sm:group-hover:bg-secondary"} />
                                    Search
                                </a>
                            </li>
                            <li>
                                <a class="border border-transparent px-3 py-1 rounded-md group sm:hover:pl-5 transition-all ease-in-out flex items-center gap-3 sm:hover:text-secondary" href="/app/contribute">
                                    <MaskedIcon src="../box.svg" class={isSmall ? "bg-white" : "sm:group-hover:bg-secondary"} />
                                    Contribute
                                </a>
                            </li>
                            <li>
                                <a class="border border-transparent px-3 py-1 rounded-md group sm:hover:pl-5 transition-all ease-in-out flex items-center gap-3 sm:hover:text-secondary" href="/app/notifications">
                                    <MaskedIcon src="../inbox.svg" class={isSmall ? "bg-white" : "sm:group-hover:bg-secondary"} />
                                    Notifications
                                </a>
                            </li>
                        </ul>
                    </div>
                {/if}
            </div>
        </div>
        <div class="rounded-xl p-10 border bg-light-background">{@render children()}</div>
    </div>
</div>
