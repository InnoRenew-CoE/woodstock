<script lang="ts">
    import Footer from "$lib/footer/Footer.svelte";
    import Header from "$lib/header/Header.svelte";
    import { notificationsStore, pushNotification } from "$lib/stores/notifications";
    import { onDestroy, onMount } from "svelte";
    import "../app.css";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { fade, slide } from "svelte/transition";
    import { verify } from "$lib";
    import { goto, invalidateAll } from "$app/navigation";

    let { children } = $props();

    let header_component: HTMLDivElement | undefined = $state(undefined);
    let footer_component: HTMLDivElement | undefined = $state(undefined);
    let layout_component: HTMLDivElement | undefined = $state(undefined);

    $effect(() => {
        let header_height = header_component?.clientHeight ?? 0;
        let footer_height = footer_component?.clientHeight ?? 0;
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
        const response = await verify();
        console.log(response);
        if (response !== 200) {
            await invalidateAll();
            await goto("/");
        }
    });
</script>

<div id="layout" bind:this={layout_component} class="bg-white text-sm min-h-[100vh] grid grid-rows-[auto_1fr_auto]">
    <div bind:this={header_component}><Header /></div>
    <div class="flex-1 h-full relative">
        {@render children()}
        <div class="absolute right-0 bottom-0 top-0 flex flex-col justify-end gap-5 p-10 pointer-events-none">
            {#each $notificationsStore as notification, i}
                <div in:slide out:slide class="{notification.class} p-2 min-w-[250px] bg-dark-background border border-secondary/80 rounded-xl shadow-lg shadow-secondary/30">
                    <div class="border border-secondary/10 bg-light-background rounded-lg px-4 py-2 flex items-center gap-3">
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
    <div bind:this={footer_component}><Footer /></div>
</div>
