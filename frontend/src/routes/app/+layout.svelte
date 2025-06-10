<script lang="ts">
    import { fade, fly, slide } from "svelte/transition";
    import { page } from "$app/stores";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { goto, invalidateAll } from "$app/navigation";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";

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
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/invalidate`, { method: "post" });
        console.log(response);
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
            <div class="flex items-center justify-center rounded-full p-4 sm:p-0">
                {#if isSmall}
                    <button onclick={() => (isVisible = !isVisible)}>
                        <MaskedIcon src="../menu.svg" class="size-4 bg-white" />
                    </button>
                {/if}
                {#if isVisible || !isSmall}
                    <div out:slide={{ duration: 100 }} class="transition-all glass p-2 rounded-full bg-white/60 hover:px-5">
                        <div class="grid sm:flex gap-3 sm:gap-2" onclick={() => (isVisible = false)}>
                            {#each paths as { link, text }}
                                {@const isSelected = link == $page.url.pathname}
                                <a class="p-1 glass rounded-full overflow-hidden px-8 {isSelected ? 'text-white bg-secondary/60 shadow-secondary/50 shadow-lg border-secondary' : 'bg-white/5 hover:bg-white'} " href={link}>
                                    {text}
                                </a>
                            {/each}
                        </div>
                    </div>
                {/if}
            </div>
            <a class="transition-all hover:gap-3 hover:px-10 group flex items-center gap-3 group relative glass bg-white p-3 px-5 rounded-full hover:bg-white/90 cursor-pointer" href="/app/feedback">
                <MaskedIcon src="/bulb.svg" class="bg-green-600 size-5" />
                Feedback
            </a>
            <div class="group transition-all glass bg-white/60 py-3 px-5 rounded-full hover:bg-white/90 cursor-pointer" onclick={logout}>
                <div class="flex items-center relative">
                    <MaskedIcon src="/logout.svg" class="bg-radial from-accent to-secondary size-5" />
                </div>
            </div>
        </div>
        <div class="rounded-xl w-full flex-1">{@render children()}</div>
    </div>
</div>
