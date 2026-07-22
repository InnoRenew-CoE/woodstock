<script lang="ts">
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    const demos = [
        {
            title: "InnoRenew - IoT Platform",
            description:
                "Our IoT platform collects and aggregates data from connected devices in real time, transforming it into clear, actionable insights. Sensor data is visualized directly on interactive floorplans, dashboards, and charts, making it easy to monitor environments, track performance, and identify issues at a glance. The platform centralizes complex device data into a simple, intuitive interface for smarter decision-making.",
            image: "https://external-content.duckduckgo.com/iu/?u=https://innorenew.eu/app/uploads/2022/12/KAB0090-2212-2000px.jpg&f=1&nofb=1&ipt=b2e6314b084fd7ae98946be765d85923f80309509a77ab286b6d3ff0de7147eb",
            links: [
                { text: "First floor", url: "https://iaq.innorenew.eu/grafana/public-dashboards/cbfa51fc8a644889844bd0ed7d313f08" },
                { text: "Ground floor", url: "https://iaq.innorenew.eu/grafana/public-dashboards/38c19ae07e8745ef83d54366c14d7b31" },
                { text: "Second floor", url: "https://iaq.innorenew.eu/grafana/public-dashboards/820b595ec2394acfb2180c4ab8be1669" },
                { text: "Third floor", url: "https://iaq.innorenew.eu/grafana/public-dashboards/556c3912bd9e42a28992983a5b4a9d3d" },
                { text: "Wall Mounted Sensors", url: "https://iaq.innorenew.eu/grafana/public-dashboards/bce5fe464a724190a3d9e3abc5dde8b5" },
            ],
        },
        // {
        //     title: "Ctfc - IoT Platform",
        //     description:
        //         "Our IoT platform collects and aggregates data from connected devices in real time, transforming it into clear, actionable insights. Sensor data is visualized directly on interactive floorplans, dashboards, and charts, making it easy to monitor environments, track performance, and identify issues at a glance. The platform centralizes complex device data into a simple, intuitive interface for smarter decision-making.",
        //     image: "./iot.jpg",
        //     link: "https://ctfc.innorenew.eu/grafana/public-dashboards/535d826acc744109b4d64a68654ee262?orgId=1&refresh=10s",
        // },
    ];
</script>

<div class="p-10 grid gap-10 sm:max-w-[80%] m-auto">
    <section class="grid gap-2">
        <h2 class="text-2xl font-semibold">Best practices</h2>
        <div class="italic text-sm">Learn about innovative solutions in wood construction through exemplary projects, design guidelines, and data from monitored buildings.</div>
    </section>

    <section class="grid gap-2 grid-cols-2">
        <a target="_blank" href="https://woodpop.eu/woodpop-dara-award/hall-of-fame/">
            <div class="hover:bg-background card p-2 flex gap-2 items-center justify-center">
                Dara Awards
                <MaskedIcon src="/external-link.svg" class="bg-info" />
            </div>
        </a>
        <a target="_blank" href="https://design-guide.build-in-wood.eu/">
            <div class="hover:bg-background card p-2 flex gap-2 items-center justify-center">
                Design Guide Build In Wood
                <MaskedIcon src="/external-link.svg" class="bg-info" />
            </div>
        </a>
    </section>
    <section class="">
        <h2 class="text-2xl font-semibold mb-6">Demo buildings using an IoT platform and deep learning</h2>

        <div class="grid gap-5 grid-cols-1 lg:grid-cols-4 items-start">
            {#each demos as card}
                <div class="bg-white rounded-lg shadow-sm overflow-hidden">
                    <!-- <img src={card.image} alt="Placeholder image" class="w-full h-[200px] object-cover border-2 rounded-lg shadow-xs border-white" /> -->

                    <div class="p-4">
                        <img src={card.image} alt="Placeholder image" class="w-full h-[200px] object-cover border-2 rounded-lg shadow-xs border-white" />

                        <h3 class="text-xl my-1">
                            {card.title}
                        </h3>

                        <p class="text-sm mb-3 line-clamp-5">
                            {card.description}
                        </p>

                        {#each card.links as link}
                            <a href={link.url} target="_blank" rel="noopener noreferrer" class="text-sm text-blue-400 hover:underline font-medium">
                                <div class="flex gap-3">
                                    <MaskedIcon src="./external-link.svg" class="bg-blue-400" />
                                    {link.text}
                                </div>
                            </a>
                        {/each}
                    </div>
                </div>
            {/each}
            <div class="bg-white rounded-lg shadow-sm overflow-hidden">
                <div class="p-4">
                    <h3 class="text-xl mb-1">More coming soon</h3>

                    <p class="text-sm mb-3 line-clamp-5">Soon more demos will be added.</p>
                </div>
            </div>
        </div>
    </section>
</div>
