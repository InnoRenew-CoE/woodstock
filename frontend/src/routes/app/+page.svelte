<script lang="ts">
    import { onMount } from "svelte";
    import * as echarts from "echarts";

    const colors = ["bg-accent", "bg-secondary", "bg-primary", "bg-orange-400", "bg-red-300"];
    let options: any[] = $state([]);
    let fileTypeChart: HTMLDivElement | undefined = $state(undefined);
    let embeddedFilesChart: HTMLDivElement | undefined = $state(undefined);
    let fileCategoriesChart: HTMLDivElement | undefined = $state(undefined);

    onMount(() => {
        type EChartsOption = echarts.EChartsOption;

        let option: EChartsOption;
        const fileTypesOption = {
            radar: {
                shape: "circle",
                radius: "50%",
                center: ["50%", "45%"],

                indicator: [
                    { name: "PDF", max: 30 },
                    { name: "Text", max: 30 },
                    { name: "Images", max: 30 },
                    { name: "Video", max: 30 },
                    { name: "CSV", max: 30 },
                ],
            },
            grid: {
                top: 50, // Padding at the top
                bottom: 0, // Padding at the bottom
                left: 50, // Padding on the left
                right: 20, // Padding on the right
            },
            series: [
                {
                    type: "radar",
                    data: [
                        {
                            value: [20, 15, 25, 10, 30],
                            areaStyle: {
                                color: "rgba(150, 128, 255, 0.3)",
                            },
                        },
                    ],
                },
            ],
        };
        const embeddedFilesOption = {
            xAxis: {
                type: "category",
                data: ["November", "December", "January"],
            },
            yAxis: {
                type: "value",
            },
            grid: {
                top: 30, // Padding at the top
                bottom: 30, // Padding at the bottom
                left: 60, // Padding on the left
                right: 50, // Padding on the right
            },
            series: [
                {
                    data: [10, 330, 1020],
                    type: "line",
                    smooth: true,
                },
            ],
        };

        const categoryFilesOption = {
            series: [
                {
                    legend: {
                        show: false, // Hide the legend
                    },
                    name: "Access From",
                    type: "pie",
                    radius: ["35%", "40%"],
                    itemStyle: {
                        borderRadius: 10,
                        borderColor: "#fff",
                        borderWidth: 1,
                    },

                    labelLine: {
                        show: true,
                    },
                    data: [
                        { value: 0.2, name: "Computer science" },
                        { value: 0.4, name: "Wood science" },
                        { value: 0.1, name: "Medicine" },
                        { value: 0.05, name: "Biology" },
                        { value: 0.05, name: "Engineering" },
                    ],
                },
            ],
        };

        options = [
            { settings: fileTypesOption, element: fileTypeChart },
            { settings: embeddedFilesOption, element: embeddedFilesChart },
            { settings: categoryFilesOption, element: fileCategoriesChart },
        ];

        rerender();
    });

    function rerender() {
        console.log("Rerender");
        for (let option of options) {
            let myChart = echarts.init(option.element, null, {
                renderer: "svg",
            });
            myChart.resize();
            myChart.setOption(option.settings);
        }
    }
</script>

<svelte:window onresize={rerender} />
<div class="h-full p-10 glass grid grid-rows-[minmax(10%,min-content)_1fr] gap-5" id="charts">
    <div class="glass px-10 py-3 flex items-center text-lg font-roboto">Dashboard</div>
    <div class="flex-1 grid gap-5 grid-cols-1 lg:grid-cols-5 grid-rows-3 sm:flex flex-wrap items-start">
        <div class="min-w-[200px] flex-1 glass p-2">
            <div class="font-roboto glass px-5 py-3">Most common file types</div>
            <div bind:this={fileTypeChart} class="chart-container"></div>
        </div>

        <div class="min-w-[200px] flex-1 glass p-2">
            <div class="font-roboto glass px-5 py-3">Embedded files</div>
            <div bind:this={embeddedFilesChart} class="chart-container"></div>
        </div>

        <div class="min-w-[200px] flex-1 glass p-2">
            <div class="font-roboto glass px-5 py-3">Sciences</div>
            <div bind:this={fileCategoriesChart} class="chart-container"></div>
        </div>
    </div>
</div>

<style>
    .chart-container {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 200px;
        min-width: 300px;
    }
    .chart-container :global(div) {
    }

    #charts :global(div > svg) {
    }
</style>
