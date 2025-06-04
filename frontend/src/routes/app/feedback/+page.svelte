<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
</script>

<div class="flex flex-wrap h-full gap-5 items-center justify-center">
    <form class="bg-dark-background p-5 rounded-lg border border-black/20 w-[90%] max-w-[800px]">
        <div class="pb-5">
            <div class="pb-3 text-amber-500 font-bold">Let us know:</div>
            <ul class="list-disc pl-8 space-y-1">
                <li>What other questions you'd think would be valuable to have on file upload</li>
                <li>What do you think about the application and the user experience</li>
                <li>Would you suggest any changes to the application design / workflow?</li>
                <li>Did you experience any issues when using the application?</li>
            </ul>
        </div>
        <textarea id="text" class="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 resize-none" rows="5" placeholder="We appreciate any kind of feedback!"></textarea>
        <div class="flex items-center justify-end py-3">
            <button class="hover:brightness-125 rounded-lg px-3 py-1 bg-orange-400 border border-orange-500 text-white">Submit</button>
        </div>
        <div class="flex items-center justify-center gap-2">
            Thank you, with <MaskedIcon src="/heart.svg" class="size-5 inline-block bg-red-400" />, team from <a href="https://innorenew.eu/sl/">InnoRenew</a>.
        </div>
    </form>
</div>
