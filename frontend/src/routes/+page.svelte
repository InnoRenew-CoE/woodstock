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
    ];

    let posts: { id: number | null; title: string; body: string; email: string; created: string }[] = $state([]);
    let isLoggedIn = $state(false);

    onMount(async () => {
        isLoggedIn = (await verify()) === 200;
        const response = await fetch(`${PUBLIC_API_BASE_URL}/posts`);
        posts = await response.json();
        posts.reverse();
        console.log(isLoggedIn);
    });

    const courses = [
        {
            title: "Introduction to Wood Material",
            description: "Introduction to Wood Material is a set of two engaging online courses introducing learners to the world of wood. Wood remains one of the most important renewable materials available to humans.",
            url: "https://neb.academy/courses/introduction-to-wood-material",
            tags: ["bio-based", "circular", "long-life"],
            photo: "https://neb.academy/images/c/1/d/d/2/c1dd2488e4b74a28d08fb56ddd32a1abcd8982a0-wood.webp",
        },
        {
            title: "Sustainability Toolbox for Building Designers",
            description: "Sustainability Toolbox for Building Designers online course equips you with the tools needed to analyze and reduce the carbon footprint throughout a building's lifecycle. The course helps you lead the transition towards sustainable design.",
            url: "https://neb.academy/courses/sustainability-toolbox-for-building-designers",
            tags: ["bio-based", "long-life"],
            photo: "https://neb.academy/images/5/d/3/c/2/5d3c236a3f0063d4636a3fd915f5c0391c2f4e54-woodenhomes.webp",
        },
        {
            title: "From Local Waste to Bio-based Innovation",
            description: "This course offers a practical, studio-based pathway for turning regional residues into high-performance, bio-based materials that advance a beautiful, sustainable, and inclusive built environment.",
            url: "https://neb.academy/courses/from-local-waste-to-bio-based-innovation",
            tags: ["bio-based", "circular", "bagaceira", "cfa"],
            photo: "https://neb.academy/images/d/3/7/f/4/d37f4fba271e3a8a8d9965aebc746656994a715d-bagaceira.jpg",
        },
        {
            title: "Circular Resource Mapping: Identifying CO₂-Absorbing Waste",
            description: "This course offers conceptual and scientific foundations to demonstrate how regional bio-based residues can support sustainable and circular innovation in construction. By-products from agriculture, forestry and industry are explored.",
            url: "https://neb.academy/courses/circular-resource-mapping",
            tags: ["bio-based", "circular", "ubi", "cfb"],
            photo: "https://neb.academy/images/c/a/6/8/2/ca682760c485c0e8940ada858a7a64eb33d38618-circularresourcemapping.png",
        },
        {
            title: "Sustainable Building with Timber",
            description: "Innovations in architecture, engineering and manufacturing make it feasible to construct almost any building in timber, including high-rise structures. Discover how this way of building could help solve the climate crisis.",
            url: "https://neb.academy/courses/tu-delft-timber",
            tags: ["bio-based", "TU-Delft"],
            photo: "https://neb.academy/images/2/a/a/4/c/2aa4c5fc07c63fa2f1edf652940c968ee29bd251-timber.webp",
        },
    ];
</script>

<div class="w-[80%] px-6 py-12 bg-white/60 rounded-2xl text-gray-800 m-auto shadow-sm grid gap-10">
    <section class="text-center">
        <div class="text-xl font-bold">The European Wood Construction Observatory</div>
        <div class="text-lg bg-gradient-to-b from-amber-700 via-yellow-800 to-amber-900 bg-clip-text text-transparent font-bold">Ask. Explore. Build better with wood.</div>
    </section>
    <section class="">
        <section class="bg-gradient-to-br from-amber-900 via-yellow-800 to-stone-900 text-amber-50 rounded-2xl p-10 relative">
            <h2 class="text-3xl md:text-4xl font-bold mb-4">European Wood Construction Observatory</h2>
            <div class="grid grid-cols-[1fr_100px]">
                <p class="text-amber-100/90 max-w-2xl mb-6 leading-relaxed">
                    An AI-driven tool for anyone looking for information on the wood and wood construction sector. Developed within the
                    <a href="http://www.woodstockproject.eu" class="underline decoration-amber-300 hover:text-white">Horizon Europe WoodStock project</a>, with open-source data and data from project partners. At a later stage, it will use data from at least three mass-timber building projects,
                    including the InnoRenew Living Lab building (Slovenia). Additional data from the University of Bordeaux and University of Galway Living Lab buildings may also be incorporated.
                </p>
                <img src="/lighthouse.png" alt="" class="rounded-xl" />
            </div>

            <ul class="grid sm:grid-cols-3 gap-4">
                <li class="bg-white/10 rounded-xl p-4">
                    <p class="font-semibold mb-1">Guidelines &amp; solutions</p>
                    <p class="text-sm text-amber-100/80">For circular and zero-waste building design.</p>
                </li>
                <li class="bg-white/10 rounded-xl p-4">
                    <p class="font-semibold mb-1">Knowledge tools</p>
                    <p class="text-sm text-amber-100/80">To expand the knowledge base in sustainable wood construction.</p>
                </li>
                <li class="bg-white/10 rounded-xl p-4">
                    <p class="font-semibold mb-1">Shared best practices</p>
                    <p class="text-sm text-amber-100/80">Collect, share, and exchange practices, including WoodStock results.</p>
                </li>
            </ul>
            <div class="flex items-center justify-center p-5">
                <a href="/search" class="px-6 py-3 rounded-full bg-gradient-to-r from-amber-600 to-amber-500 text-amber-50 font-semibold shadow-md hover:shadow-lg hover:from-amber-600 hover:to-amber-800 active:scale-95 transition-all duration-200"> Ask &amp; Explore here </a>
            </div>
        </section>
    </section>
    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h1 class="text-3xl font-bold mb-4">Latest posts</h1>
        {#if posts.length > 0}
            <div class="grid gap-5 md:grid-cols-3 place-items-stretch">
                {#each posts as post}
                    <a target="_blank" href="/post/{post.id}" class=" bg-white rounded-lg border border-teal-800/10 overflow-hidden hover:shadow-lg transition-shadow duration-300">
                        <div class="p-6">
                            <div class="flex justify-between items-center">
                                <h2 class="text-xl text-accent mb-3 line-clamp-1">{post.title}</h2>
                                {#if isLoggedIn}
                                    <a href="/app/collaborate?id={post.id}" target="_blank">
                                        <MaskedIcon class="hover:bg-secondary bg-primary size-5" src="./edit.svg" />
                                    </a>
                                {/if}
                            </div>
                            <div class="text-gray-600 text-sm leading-relaxed line-clamp-2 w-full">{@html post.body}</div>
                            <div class="pt-5 flex justify-between items-center text-xs">
                                <span class="text-secondary">{post.email}</span><span class="text-secondary-1">{post.created}</span>
                            </div>
                        </div>
                    </a>
                {/each}
            </div>
        {:else}
            <div class="text-sm text-center text-accent">No posts yet.</div>
        {/if}
    </section>

    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our courses (NEB Academy)</h2>
        <div class="grid md:grid-cols-3 gap-3">
            {#each courses as course}
                <a target="_blank" href={course.url} class="hover:shadow hover:brightness-95 bg-white rounded-xl p-5">
                    {#if course.photo}
                        <img src={course.photo} class="rounded-lg" />
                    {/if}
                    <div class="text-lg font-bold">{course.title}</div>
                    <div class="text-xs text-secondary-1">{course.tags.join(" ")}</div>
                    <div class="text-sm py-3 text-primary">{course.description}</div>
                </a>
            {/each}
            <a href="/about" class="p-10 bg-accent/20 rounded-lg shadow flex items-center justify-center text-amber-800 border border-accent/30 underline underline-offset-2 hover:text-amber-950">Find more</a>
        </div>
    </section>
</div>
