<script>
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { pushNotification } from "$lib/stores/notifications";
    import { preventDefault } from "svelte/legacy";

    let text = $state("");
    async function submit() {
        if (text.trim().length > 10) {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/api/feedback`, {
                method: "post",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(text),
            });
            if (response.status === 200) {
                pushNotification({ title: "Feedback sent", body: "Thank you very much!" });
                text = "";
            } else {
                pushNotification({ body: "An error occured, please notify us at woodstock@innorenew.eu", title: "Request failed" });
            }
        } else {
            pushNotification({ title: "Feedback details", body: "Please provide more details with your feedback." });
        }
    }
</script>

<div class="flex flex-wrap h-full gap-5 items-center justify-center">
    <form class="glass flex flex-col gap-5 p-10 w-[90%] max-w-[1024px]" onsubmit={submit}>
        <div class="glass p-5 bg-white/80">
            <div class="pb-3 text-indigo-500 font-mono text-lg">Let us know:</div>
            <ul class="list-disc pl-8 space-y-1">
                <li>What other questions you'd think would be valuable to have on file upload</li>
                <li>What do you think about the application and the user experience</li>
                <li>Would you suggest any changes to the application design / workflow?</li>
                <li>Did you experience any issues when using the application?</li>
            </ul>
        </div>
        <textarea bind:value={text} id="text" class="glass p-5 resize-none" rows="5" placeholder="We appreciate any kind of feedback!"></textarea>
        <div class="flex items-center justify-end py-3">
            <button type="submit" class="transition-all cursor-pointer hover:bg-secondary/10 hover:border-secondary/50 rounded-lg glass px-5 py-2">Submit</button>
        </div>
        <div class="flex items-center justify-center gap-1">
            <span>Thank you, with</span><MaskedIcon src="/heart.svg" class="size-5 inline-block bg-red-400" /><span> from <a target="_blank" class="text-accent" href="https://innorenew.eu/sl/">InnoRenew</a>.</span>
        </div>
    </form>
</div>
