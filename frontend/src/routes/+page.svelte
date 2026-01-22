<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";

    const cards = [
        {
            title: "TIMBERHAUS Focus: Sustainable Wood Construction & Circular Economy",
            description:
                "TIMBERHAUS is a European innovation project dedicated to reducing the construction sector’s carbon footprint by developing climate-smart, circular solutions for wood construction. Addressing the fact that the building industry accounts for 40% of global CO₂ emissions and 35% of waste, TIMBERHAUS focuses on increasing the use of wood as a primary, sustainable material. The project is currently validating its technologies and strategies in three diverse pilot cities—Berlin (Germany), Baia Mare (Romania), and Siena (Italy)—to demonstrate how timber can be integrated into different urban contexts and heritage settings.",
            image: "./timberhaus.svg",
            link: "https://timberhaus.eu",
        },
        {
            title: "IoT Platform",
            description: "Duis sagittis ipsum. Praesent mauris. Fusce nec tellus sed augue semper porta.",
            image: "./iot.jpg",
            link: "https://iaq.innorenew.eu/grafana/public-dashboards/535d826acc744109b4d64a68654ee262?orgId=1&refresh=10s",
        },
        {
            title: "Woodstock",
            description: "This platform.",
            image: "./platform.jpg",
            link: "/login",
        },
    ];
</script>

<div class="max-w-6xl mx-auto px-6 py-12 bg-white/60 rounded-lg text-gray-800 w-[80%] m-auto shadow-sm">
    <!-- About the Project -->
    <section class="mb-12">
        <h1 class="text-3xl font-bold mb-4">About</h1>
        <p class="text-lg leading-relaxed">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
    </section>

    <!-- Cards Section -->
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our projects</h2>

        <div class="grid gap-10">
            {#each cards as card}
                <div class="bg-white rounded-lg shadow-sm overflow-hidden">
                    <img src={card.image} alt="Placeholder image" class="w-full h-[500px] object-cover border-2 rounded-lg shadow-xs border-white" />

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
