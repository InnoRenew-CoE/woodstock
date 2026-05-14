<script lang="ts">
    import { page } from "$app/state";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { pushNotification } from "$lib/stores/notifications";
    import { Tipex } from "@friendofsvelte/tipex";
    import "@friendofsvelte/tipex/styles/index.css";
    import type { Editor } from "@tiptap/core";
    import { onMount } from "svelte";

    let editor: Editor | undefined = $state();

    const id = $derived(parseInt(page.url.searchParams.get("id")));

    // Reactive HTML content
    const htmlContent = $derived(editor?.getHTML() ?? "");

    // Reactive text content (no HTML tags)
    const textContent = $derived(editor?.getText() ?? "");
    let title = $state("");

    // Reactive word count
    const wordCount = $derived(
        editor
            ?.getText()
            .split(/\s+/)
            .filter((word) => word.length > 0).length ?? 0,
    );

    async function submit() {
        if (title.length < 5 || htmlContent.length < 10) {
            return;
        }
        const body = {
            id: id,
            title: title,
            body: htmlContent,
        };
        console.log(body);
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/collaborate`, {
            method: "post",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        });
        pushNotification({ title: "Post successful", body: "Your submission has been successful." });
        console.log(await response.json());
    }

    onMount(async () => {
        if (id) {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/posts`);
            const posts: { id: number; title: string; body: string }[] = await response.json();
            const post = posts.find((p) => p.id === parseInt(id));
            if (post) {
                editor?.commands.setContent(post.body);
                title = post.title;
            }
        }
    });
</script>

<div class="hidden md:block">
    <div class="editor-stats flex gap-1 items-center justify-start p-2">
        <p class="p-2 bg-white/30 rounded-lg text-xs">Words: {wordCount}</p>
        <p class="p-2 bg-white/30 rounded-lg text-xs">Characters: {textContent.length}</p>
    </div>

    <div class="grid grid-cols-2 gap-5">
        <div>
            <input bind:value={title} required placeholder="Title" class="bg-white/50 p-3 w-full rounded-lg border-gray-200 border" />
            <Tipex bind:tipex={editor} floating focal class="border border-neutral-200/80" />
            <div class="flex items-center justify-end py-5">
                <button onclick={submit} disabled={textContent.length < 10 || title.length < 5} type="submit" class="bg-primary disabled:bg-gray-300 text-white px-3 py-1 rounded">Submit</button>
            </div>
        </div>
        <div class="prose">
            <div class="text-xs -mt-5">Preview</div>
            <div class="font-bold text-lg">{title}</div>
            {@html htmlContent}
        </div>
    </div>
</div>
<div class="block md:hidden text-center">Submitting a post is only available on bigger screens.</div>
