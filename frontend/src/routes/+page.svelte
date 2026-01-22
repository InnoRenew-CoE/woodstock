<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    const cards = [
        {
            title: "Lorem Ipsum",
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
            image: "https://placehold.co/600x400",
            link: "https://example.com",
        },
        {
            title: "Dolor Sit Amet",
            description: "Sed cursus ante dapibus diam. Sed nisi. Nulla quis sem at nibh elementum imperdiet.",
            image: "https://placehold.co/600x400",
            link: "https://example.com",
        },
        {
            title: "Consectetur",
            description: "Duis sagittis ipsum. Praesent mauris. Fusce nec tellus sed augue semper porta.",
            image: "https://placehold.co/600x400",
            link: "https://example.com",
        },
    ];
</script>

<div class="max-w-6xl mx-auto px-6 py-12 bg-white/60 rounded-lg text-gray-800 w-[80%] m-auto shadow-sm">
    <!-- About the Project -->
    <section class="mb-12">
        <h1 class="text-3xl font-bold mb-4">About the Project</h1>
        <p class="text-lg leading-relaxed">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
    </section>

    <!-- Cards Section -->
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our projects</h2>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each cards as card}
                <div class="bg-white rounded-lg shadow-sm overflow-hidden">
                    <img src={card.image} alt="Placeholder image" class="w-full h-40 object-cover border-2 rounded-lg shadow-xs border-white" />

                    <div class="p-4">
                        <h3 class="text-xl mb-1">
                            {card.title}
                        </h3>

                        <p class="text-sm mb-3">
                            {card.description}
                        </p>

                        <a href={card.link} target="_blank" rel="noopener noreferrer" class="text-sm text-blue-400 hover:underline font-medium">
                            <div class="flex gap-3">
                                <MaskedIcon src="./external-link.svg" class="bg-blue-400" />
                                Visit external page
                            </div>
                        </a>
                    </div>
                </div>
            {/each}
        </div>
    </section>
</div>
