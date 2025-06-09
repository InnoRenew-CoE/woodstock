<script lang="ts">
    import { fade, slide } from "svelte/transition";
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { goto, invalidateAll } from "$app/navigation";

    let { children } = $props();
    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);

    const paths: { link: string; text: string }[] = [
        { link: "/app", text: "Home" },
        { link: "/app/search", text: "Search" },
        { link: "/app/contribute", text: "Contribute" },
        { link: "/app/notifications", text: "Notifications" },
        { link: "/app/feedback", text: "Feedback" },
    ];
    let notificationCount = $state(0);

    async function logout() {
        document.cookie = "access_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        document.cookie = "refresh_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        await invalidateAll();
        await goto("/");
    }
</script>

<svelte:window bind:innerWidth={width} />

<div class="p-2 sm:p-5 h-full">
    <div class="flex flex-col items-center h-full gap-5">
        <div class="p-3 flex gap-3 items-center justify-center relative w-full">
            <div class="p-2 bg-white backdrop-blur-2xl border-white rounded-full">
                <img src="/woodstock.svg" class="size-8 rounded-full" />
            </div>
            <div class="flex items-center justify-center bg-black rounded-full p-4 sm:p-1">
                {#if isSmall}
                    <button onclick={() => (isVisible = !isVisible)}>
                        <MaskedIcon src="../menu.svg" class="size-4 bg-white" />
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
                            {#each paths as { link, text }}
                                {@const isSelected = link == $page.url.pathname}
                                <li>
                                    <a class="{text === 'Feedback' ? 'bg-orange-400' : ''} cursor-pointer flex items-center gap-3 {isSelected ? 'bg-white text-black' : ''} px-8 py-1.5 rounded-full" href={link}>
                                        {text}
                                    </a>
                                </li>
                            {/each}
                            <div class="flex items-center gap-3 text-red-400 group relative" onclick={logout}>
                                <MaskedIcon src="/logout.svg" class="group-hover:bg-red-400 bg-white size-5" />
                            </div>
                        </ul>
                    </div>
                {/if}
            </div>
        </div>
        <div class="rounded-xl p-10 bg-light-background w-full flex-1 border border-secondary/30 shadow-sm">{@render children()}</div>
    </div>
</div>
