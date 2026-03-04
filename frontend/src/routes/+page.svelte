<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { verify } from "$lib";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { onMount } from "svelte";

    const cards = [
        {
            title: "Woodstock",
            description:
                "Our platform helps researchers collaborate more effectively by turning uploaded research files into a shared, searchable knowledge system. Instead of manually reviewing documents, users can ask questions and receive clear answers about ongoing projects, progress updates, and related work.                 By understanding the meaning behind the content—not just keywords—the system makes it easy to track developments, discover collaboration opportunities, and stay informed about fellow researchers’ work. The goal is to make research more connected, accessible, and efficient.",
            image: "./platform.png",
            link: "/login",
        },
        {
            title: "TIMBERHAUS Focus: Sustainable Wood Construction & Circular Economy",
            description:
                "TIMBERHAUS is a European innovation project dedicated to reducing the construction sector’s carbon footprint by developing climate-smart, circular solutions for wood construction. Addressing the fact that the building industry accounts for 40% of global CO₂ emissions and 35% of waste, TIMBERHAUS focuses on increasing the use of wood as a primary, sustainable material. The project is currently validating its technologies and strategies in three diverse pilot cities—Berlin (Germany), Baia Mare (Romania), and Siena (Italy)—to demonstrate how timber can be integrated into different urban contexts and heritage settings.",
            image: "./timberhaus.svg",
            link: "https://timberhaus.eu",
        },

        {
            title: "IoT Platform",
            description:
                "Our IoT platform collects and aggregates data from connected devices in real time, transforming it into clear, actionable insights. Sensor data is visualized directly on interactive floorplans, dashboards, and charts, making it easy to monitor environments, track performance, and identify issues at a glance. The platform centralizes complex device data into a simple, intuitive interface for smarter decision-making.",
            image: "./iot.jpg",
            link: "https://iaq.innorenew.eu/grafana/public-dashboards/535d826acc744109b4d64a68654ee262?orgId=1&refresh=10s",
        },
    ];

    let posts: { id: number | null; title: string; body: string; author: string; created: string }[] = $state([]);
    let isLoggedIn = $state(false);

    onMount(async () => {
        isLoggedIn = (await verify()) === 200;
        const response = await fetch(`${PUBLIC_API_BASE_URL}/posts`);
        posts = await response.json();
    });
</script>

<div class="w-[80%] px-6 py-12 bg-white/60 rounded-2xl text-gray-800 m-auto shadow-sm grid gap-10">
    <section>
        <h1 class="text-3xl font-bold mb-4">About</h1>
        <div class="grid gap-3 text-lg leading-relaxed">
            <b>Our platform is designed to accelerate research through intelligent, secure collaboration.</b>

            <div>We provide a shared digital environment where researchers can upload papers, datasets, reports, and working documents—transforming static files into a dynamic, searchable knowledge network.</div>
            <div>
                By converting uploaded research materials into rich semantic representations, our system understands the meaning and context behind the content, not just keywords. This enables researchers to ask natural-language questions about ongoing work and receive clear, contextual answers
                drawn directly from the collective research repository.
            </div>
        </div>
    </section>
    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h1 class="text-3xl font-bold mb-4">Latest posts</h1>
        <div class="grid gap-5 grid-cols-4">
            {#each posts as post}
                <a target="_blank" href="/post/{post.id}">
                    <div class=" bg-white rounded-lg border border-teal-800/10 overflow-hidden hover:shadow-lg transition-shadow duration-300">
                        <div class="p-6">
                            <div class="flex justify-between items-center">
                                <h2 class="text-xl font-semibold text-gray-800 mb-3">{post.title}</h2>
                                {#if isLoggedIn}
                                    <a href="/app/collaborate?id={post.id}" target="_blank">
                                        <MaskedIcon class="hover:bg-secondary bg-primary size-5" src="./edit.svg" />
                                    </a>
                                {/if}
                            </div>
                            <div class="text-gray-600 text-sm leading-relaxed line-clamp-2 w-full">{@html post.body}</div>
                            <div class="pt-5 flex justify-between items-center">
                                <span class="text-secondary">{post.author}</span><span class="text-secondary-1">{post.created}</span>
                            </div>
                        </div>
                    </div>
                </a>
            {/each}
        </div>
    </section>
    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our projects</h2>

        <div class="grid gap-10 grid-cols-1 md:grid-cols-2">
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
