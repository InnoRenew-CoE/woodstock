<script>
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { verify } from "$lib";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";

    let width = $state(0);
    let isSmall = $derived(width < 640);
    let isVisible = $state(false);

    const paths = [
        { text: "Home", url: "/", icon: "/home.svg" },
        // { text: "About", url: "/about", icon: "/heart.svg" },
        { text: "Chat", url: "/search", icon: "/search.svg" },
        // { text: "Login", url: "/login", icon: "/home.svg" },
    ];

    let isLoggedIn = $state(false);
    onMount(async () => {
      console.log("Header mount");
        isLoggedIn = (await verify()) === 200;
    });

    async function logout() {
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/invalidate`, { method: "post" });
        console.log(response);
        await invalidateAll();
        await goto("/", { invalidateAll: true });
        isLoggedIn = (await verify()) === 200;
    }
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
                        {#if isLoggedIn}
                            <div
                                onclick={async () => await goto("/app")}
                                class="group transition-all glass bg-white/60 py-2 px-3 rounded-full hover:bg-white/90 cursor-pointer bg-black sm:bg-white hover:!opacity-100 hover:text-white hover:bg-secondary/60 hover:shadow-secondary/50 hover:shadow-lg hover:border-secondary sm:text-black"
                            >
                                <div class="flex items-center relative text-primary">App</div>
                            </div>
                            <div class="group transition-all glass bg-white/60 py-2 px-3 rounded-full hover:bg-white/90 cursor-pointer" onclick={logout}>
                                <div class="flex items-center relative">
                                    <MaskedIcon src="/logout.svg" class="bg-radial from-accent to-secondary size-5" />
                                </div>
                            </div>
                        {:else}
                            <a
                                class="cursor-pointer flex items-center gap-5 sm:opacity-50 p-1.5 glass rounded-full overflow-hidden px-8 border-white/20 'bg-black sm:bg-white hover:!opacity-100 hover:text-white hover:bg-secondary/60 hover:shadow-secondary/50 hover:shadow-lg hover:border-secondary sm:text-black"
                                href="/login"
                            >
                                <MaskedIcon src="./home.svg" class={isSmall ? "bg-white" : "hidden"} />{"Login"}
                            </a>
                        {/if}
                    </div>
                </div>
            {/if}
        </div>
    </div>
{/if}
