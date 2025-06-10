<script lang="ts">
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { goto, invalidateAll } from "$app/navigation";

    let { children } = $props();
    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);
    let exitHover = $state(false);

    const paths: { link: string; text: string }[] = [
        { link: "/app", text: "Home" },
        { link: "/app/search", text: "Search" },
        { link: "/app/contribute", text: "Contribute" },
        { link: "/app/contributions", text: "Contributions" },
        { link: "/app/notifications", text: "Notifications" },
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
            <div class="glass bg-white/60 p-2 rounded-full hover:bg-white/70 cursor-pointer">
                <img src="/woodstock.svg" class="size-7 rounded-full" />
            </div>
            <div class="flex items-center justify-center rounded-full p-4 sm:p-0 bg-black">
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
                                    <a class="cursor-pointer flex items-center gap-3 {isSelected ? 'bg-white text-black' : ''} px-8 py-1.5 rounded-full hover:brightness-90" href={link}>
                                        {text}
                                    </a>
                                </li>
                            {/each}
                        </ul>
                    </div>
                {/if}
            </div>
            <a class="transition-all hover:gap-3 hover:px-10 group flex items-center gap-3 group relative glass bg-white p-3 px-5 rounded-full hover:bg-white/90 cursor-pointer" href="/app/feedback">
                <MaskedIcon src="/bulb.svg" class="bg-green-600 size-5" />
                Feedback
            </a>
            <div class="group transition-all glass bg-white/60 py-3 px-5 rounded-full hover:bg-white/90 cursor-pointer">
                <div class="flex items-center relative" onclick={logout}>
                    <MaskedIcon src="/logout.svg" class="bg-radial from-accent to-secondary size-5" />
                </div>
            </div>
        </div>
        <div class="rounded-xl w-full flex-1">{@render children()}</div>
    </div>
</div>
