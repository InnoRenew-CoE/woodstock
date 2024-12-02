<script lang="ts">
    import Footer from "$lib/footer/Footer.svelte";
    import Header from "$lib/header/Header.svelte";
    import "../app.css";
    let { children } = $props();
    let header_component: HTMLDivElement | undefined = $state(undefined);
    let footer_component: HTMLDivElement | undefined = $state(undefined);
    let layout_component: HTMLDivElement | undefined = $state(undefined);

    $effect(() => {
        let header_height = header_component?.clientHeight ?? 0;
        let footer_height = footer_component?.clientHeight ?? 0;
        if (layout_component) {
            layout_component.style.display = "grid";
            layout_component.style.gridTemplateRows = `auto minmax(calc(100vh - ${header_height}px - ${footer_height}px), auto) ${footer_height}px`;

            /*
            Calculation is the following: 2rem header, 100vh - (footer + header) for content.
            This way the content is either full screen or more.
            */
        }
    });
</script>

<div id="layout" bind:this={layout_component}>
    <div bind:this={header_component}><Header /></div>
    <div>{@render children()}</div>
    <div bind:this={footer_component}><Footer /></div>
</div>
