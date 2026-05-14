<script lang="ts">
    import { page } from "$app/state";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { onMount } from "svelte";
    const id = $derived(parseInt(page.params.id));
    let post: { id: number | null; title: string; body: string; email: string; created: string } | undefined = $state(undefined);

    onMount(async () => {
        let response = await fetch(`${PUBLIC_API_BASE_URL}/posts`);
        let posts: { id: number | null; title: string; body: string; email: string; created: string }[] = await response.json();
        console.log(posts);
        post = posts.find((p) => p.id === id);
    });
</script>

<div class="bg-white w-[90%] m-auto rounded-lg p-10">
    {#if post}
        <div class="flex gap-3 items-center justify-between">
            <div class="text-xl font-bold">{post.title}</div>
            <div class="text-xs flex gap-3">
                Author:
                <div class=" text-accent">{post.email.split("@")[0]}</div>
                Date:
                <div class=" text-secondary">{post.created}</div>
            </div>
        </div>
        <div class="prose">
            {@html post.body}
        </div>
    {/if}
</div>
