<script lang="ts">
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { slide } from "svelte/transition";

    let { children } = $props();
    let width = $state(0);
    let isSmall = $derived(width < 1024);
    let isVisible = $state(false);
    let exitHover = $state(false);

    const paths: { link: string; text: string }[] = [
        { link: "/app", text: "Home" },
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

<div class="p-5 h-full">
    <div class="flex flex-col items-center h-full gap-5">
        <div class="p-3 flex gap-3 items-center justify-center w-full relative">
            <div class="glass bg-white/60 p-2 rounded-full hover:bg-white/70 cursor-pointer">
                <img src="/woodstock.svg" class="size-7 rounded-full" />
            </div>
            <div class="flex items-center justify-center rounded-full p-0 lg:p-0">
                {#if isSmall}
                    <div class="glass rounded-full p-3 bg-white/40" onclick={() => (isVisible = !isVisible)}>
                        <MaskedIcon src="../menu.svg" class="size-5 bg-gradient-to-b from-secondary to-secondary/50" />
                    </div>
                {/if}
                {#if isVisible || !isSmall}
                    <div out:slide={{ duration: 100 }} class="z-5 backdrop-blur-sm absolute left-0 right-0 top-[90%] lg:relative transition-all glass p-1.5 lg:rounded-full bg-white/60 hover:px-5">
                        <div class="grid lg:flex gap-3" onclick={() => (isVisible = false)}>
                            {#each paths as { link, text }}
                                {@const isSelected = link == $page.url.pathname}
                                <a
                                    draggable={false}
                                    class="transition-all p-1.5 glass rounded-xl lg:rounded-full overflow-hidden px-8 {isSelected ? 'text-white bg-secondary/60 shadow-secondary/50 shadow-lg border-secondary' : 'bg-white/5 backdrop-blur-none shadow-transparent border-transparent hover:bg-white'} "
                                    href={link}
                                >
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
