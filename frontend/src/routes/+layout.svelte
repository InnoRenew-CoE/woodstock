<script lang="ts">
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/state";
    import { verify } from "$lib";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import Header from "$lib/header/Header.svelte";
    import { notificationsStore } from "$lib/stores/notifications";
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    import "../app.css";

    let { children } = $props();

    let header_component: HTMLElement | undefined = $state(undefined);
    let footer_component: HTMLDivElement | undefined = $state(undefined);
    let layout_component: HTMLDivElement | undefined = $state(undefined);

    $effect(() => {
        // let header_height = header_component?.clientHeight ?? 0;
        // let footer_height = footer_component?.clientHeight ?? 0;
        if (layout_component) {
            // layout_component.style.display = "grid";
            // layout_component.style.gridTemplateRows = `auto minmax(calc(100vh - ${header_height}px - ${footer_height}px), auto) ${footer_height}px`;
            /*
            Calculation is the following: 2rem header, 100vh - (footer + header) for content.
            This way the content is either full screen or more.
            */
        }
    });

    onMount(async () => {
        const isApp = page.url.pathname.includes("app");
        if (isApp) {
            const response = await verify();
            console.log(response);
            if (response !== 200) {
                await invalidateAll();
                await goto("/");
            }
        }
    });
</script>

<header bind:this={header_component} class="font-nunito">
    <Header />
</header>
<div id="layout" bind:this={layout_component} class="font-roboto grid grid-rows-[auto_1fr_auto]">
    <div class="flex-1 h-full relative py-5">
        {@render children()}
        <div class="fixed right-0 bottom-0 top-0 flex flex-col justify-end gap-5 p-10 pointer-events-none">
            {#each $notificationsStore as notification, i}
                <div in:slide out:slide class="{notification.class} p-2 min-w-[250px] glass">
                    <div class="glass rounded-lg px-4 py-2 flex items-center gap-3">
                        <MaskedIcon src="../bell.svg" class="{notification.class} size-5 bg-secondary" />
                        <div>
                            <div class="font-bold">{notification.title}</div>
                            <div class="font-light">{notification.body}</div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
