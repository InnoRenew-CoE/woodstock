<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { onMount } from "svelte";

    type FileTypeStatistic = { file_type: string; count: number; percentage: number };
    type TagStatistic = { tag: string; count: number };
    type MonthlyUpload = { month: string; count: number };
    type RecentUpload = { name: string; file_type: string; date: string | null; status: string; submitted_by: string | null };
    type Statistics = {
        total_files: number;
        total_users: number;
        active_users: number;
        total_tags: number;
        total_answers: number;
        total_posts: number;
        total_feedback: number;
        total_queries: number;
        total_logins: number;
        processed_files: number;
        pending_files: number;
        failed_files: number;
        files_last_week: number;
        my_files: number;
        my_files_this_month: number;
        my_answers: number;
        file_types: FileTypeStatistic[];
        top_tags: TagStatistic[];
        monthly_uploads: MonthlyUpload[];
        recent_uploads: RecentUpload[];
    };

    const unknownStatistics: Statistics = {
        total_files: 0,
        total_users: 0,
        active_users: 0,
        total_tags: 0,
        total_answers: 0,
        total_posts: 0,
        total_feedback: 0,
        total_queries: 0,
        total_logins: 0,
        processed_files: 0,
        pending_files: 0,
        failed_files: 0,
        files_last_week: 0,
        my_files: 0,
        my_files_this_month: 0,
        my_answers: 0,
        file_types: [],
        top_tags: [],
        monthly_uploads: [],
        recent_uploads: [],
    };

    let statistics = $state<Statistics>(unknownStatistics);
    let loading = $state(true);
    let error = $state<string | null>(null);
    let updatedAt = $state<Date | null>(null);

    const numberFormat = new Intl.NumberFormat();

    let pipeline = $derived.by(() => {
        const done = statistics.processed_files;
        const pending = statistics.pending_files;
        const failed = statistics.failed_files;
        const total = done + pending + failed;
        const percentOf = (value: number) => (total > 0 ? (value / total) * 100 : 0);
        return {
            total,
            done,
            pending,
            failed,
            donePercent: percentOf(done),
            pendingPercent: percentOf(pending),
            failedPercent: percentOf(failed),
            successRate: total > 0 ? Math.round((done / total) * 100) : 0,
        };
    });

    let monthly = $derived.by(() => {
        const rows = statistics.monthly_uploads;
        const max = rows.reduce((peak, row) => Math.max(peak, row.count), 0);
        const total = rows.reduce((sum, row) => sum + row.count, 0);
        return { rows, max, total };
    });

    let topTag = $derived(statistics.top_tags[0] ?? null);
    let myShare = $derived(statistics.total_files > 0 ? Math.round((statistics.my_files / statistics.total_files) * 1000) / 10 : 0);

    const statusStyles: Record<string, string> = {
        done: "bg-emerald-100/60 text-emerald-600 border-emerald-200",
        failed: "bg-rose-100/60 text-rose-500 border-rose-200",
        uploaded: "bg-amber-100/50 text-amber-600 border-amber-200",
        pending: "bg-amber-100/50 text-amber-600 border-amber-200",
    };

    function formatNumber(value: number): string {
        return numberFormat.format(value);
    }

    function percent(value: number): string {
        return `${Number(value.toFixed(1))}%`;
    }

    function monthLabel(value: string): string {
        const [year, month] = value.split("-").map(Number);
        return new Date(year ?? 1970, (month ?? 1) - 1, 1).toLocaleDateString(undefined, { month: "long", year: "numeric" });
    }

    function dateLabel(value: string | null): string {
        if (!value) return "—";
        const [year, month, day] = value.slice(0, 10).split("-").map(Number);
        return new Date(year ?? 1970, (month ?? 1) - 1, day ?? 1).toLocaleDateString(undefined, {
            day: "2-digit",
            month: "short",
            year: "numeric",
        });
    }

    async function load() {
        loading = true;
        error = null;
        try {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/api/statistics`);
            if (!response.ok) {
                throw new Error(`Request failed with status ${response.status}`);
            }
            statistics = (await response.json()) as Statistics;
            updatedAt = new Date();
        } catch (err) {
            error = err instanceof Error ? err.message : "Unable to load statistics.";
        } finally {
            loading = false;
        }
    }

    onMount(load);
</script>

{#snippet statCard(label: string, value: number, hint: string, icon: string, tone: string)}
    <div class="glass p-5 flex flex-col gap-3 transition-all hover:-translate-y-0.5 hover:bg-white/30">
        <div class="flex items-center justify-between">
            <span class="text-[11px] font-mono uppercase tracking-[0.18em] opacity-45">{label}</span>
            <MaskedIcon src={icon} class="size-4 {tone}" />
        </div>
        <div class="text-4xl font-roboto leading-none">{formatNumber(value)}</div>
        <div class="text-xs opacity-50 leading-snug">{hint}</div>
    </div>
{/snippet}

{#snippet tableHeader(title: string, caption: string, icon: string)}
    <div class="flex items-center justify-between gap-3">
        <h2 class="font-roboto text-lg leading-none">{title}</h2>
        <div class="flex items-center gap-2 text-[11px] font-mono uppercase tracking-wider opacity-40">
            <MaskedIcon src={icon} class="size-3 bg-primary/50" />
            {caption}
        </div>
    </div>
{/snippet}

<div class="h-full min-h-0 flex flex-col gap-5">
    <header class="glass px-6 py-4 flex flex-wrap items-center justify-between gap-4">
        <div class="flex flex-col">
            <h1 class="text-2xl font-roboto leading-tight">Dashboard</h1>
            <p class="text-[11px] font-mono uppercase tracking-[0.2em] opacity-40">Live overview of the Woodstock corpus</p>
        </div>
        <div class="flex items-center gap-3">
            {#if updatedAt}
                <span class="hidden sm:inline text-[11px] font-mono uppercase tracking-wider opacity-40">
                    Updated {updatedAt.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}
                </span>
            {/if}
            <button
                class="flex items-center gap-2 glass bg-secondary/10 border-secondary/40 px-4 py-2 text-sm text-secondary transition-all hover:bg-secondary/20 disabled:opacity-50"
                onclick={load}
                disabled={loading}
            >
                <MaskedIcon src="/loading.svg" class="size-4 bg-secondary {loading ? 'animate-spin' : 'hidden'}" />
                {loading ? "Loading" : "Refresh"}
            </button>
        </div>
    </header>

    <div class="flex-1 min-h-0 overflow-auto pr-1 pb-2 flex flex-col gap-5">
        {#if loading && !updatedAt}
            <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
                {#each [0, 1, 2, 3] as _}
                    <div class="glass h-[136px] animate-pulse bg-white/10"></div>
                {/each}
            </div>
            <div class="grid gap-4 lg:grid-cols-3">
                <div class="glass h-[190px] animate-pulse bg-white/10 lg:col-span-2"></div>
                <div class="glass h-[190px] animate-pulse bg-white/10"></div>
            </div>
            <div class="glass h-[300px] animate-pulse bg-white/10"></div>
        {:else if error}
            <div class="glass p-10 flex flex-col items-center justify-center gap-3 text-center">
                <MaskedIcon src="/info.svg" class="size-8 bg-destructive/70" />
                <p class="font-roboto text-lg">Statistics are unavailable</p>
                <p class="text-sm opacity-50 max-w-md">{error}. Please make sure you are signed in and that the API is reachable.</p>
                <button class="mt-2 glass bg-white/40 px-5 py-2 text-sm hover:bg-white/60" onclick={load}>Try again</button>
            </div>
        {:else}
            <!-- Primary KPIs -->
            <section class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
                {@render statCard(
                    "Files curated",
                    statistics.total_files,
                    `${formatNumber(statistics.pending_files)} pending · ${formatNumber(statistics.files_last_week)} added in the last 7 days`,
                    "/box.svg",
                    "bg-secondary",
                )}
                {@render statCard(
                    "Contributors",
                    statistics.active_users,
                    `${formatNumber(statistics.total_users)} registered accounts`,
                    "/heart.svg",
                    "bg-accent",
                )}
                {@render statCard(
                    "Answers captured",
                    statistics.total_answers,
                    `${formatNumber(statistics.total_queries)} assistant queries logged`,
                    "/book-open.svg",
                    "bg-secondary-1",
                )}
                {@render statCard(
                    "Tags in use",
                    statistics.total_tags,
                    topTag ? `Most used: ${topTag.tag} (${formatNumber(topTag.count)})` : "No tags assigned yet",
                    "/archive.svg",
                    "bg-secondary-2",
                )}
            </section>

            <!-- Activity + pipeline -->
            <section class="grid gap-4 lg:grid-cols-3">
                <div class="glass p-5 flex flex-col gap-4 lg:col-span-2">
                    <div class="flex items-center justify-between gap-3">
                        <h2 class="font-roboto text-lg leading-none">Your contributions</h2>
                        <MaskedIcon src="/email.svg" class="size-4 bg-accent" />
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                        <div class="glass bg-white/30 px-4 py-3 flex flex-col gap-1">
                            <span class="text-3xl font-roboto leading-none">{formatNumber(statistics.my_files)}</span>
                            <span class="text-[11px] uppercase tracking-wider opacity-45">files submitted</span>
                        </div>
                        <div class="glass bg-white/30 px-4 py-3 flex flex-col gap-1">
                            <span class="text-3xl font-roboto leading-none text-secondary">{formatNumber(statistics.my_files_this_month)}</span>
                            <span class="text-[11px] uppercase tracking-wider opacity-45">this month</span>
                        </div>
                        <div class="glass bg-white/30 px-4 py-3 flex flex-col gap-1">
                            <span class="text-3xl font-roboto leading-none">{formatNumber(statistics.my_answers)}</span>
                            <span class="text-[11px] uppercase tracking-wider opacity-45">answers given</span>
                        </div>
                    </div>
                    <p class="text-xs opacity-45">
                        Your files make up <span class="font-mono text-secondary">{percent(myShare)}</span> of the whole corpus.
                        The community has shared {formatNumber(statistics.total_posts)} posts and {formatNumber(statistics.total_feedback)} pieces of feedback.
                    </p>
                </div>

                <div class="glass p-5 flex flex-col gap-4">
                    <div class="flex items-center justify-between gap-3">
                        <h2 class="font-roboto text-lg leading-none">Embedding pipeline</h2>
                        <MaskedIcon src="/tree_of_knowledge.svg" class="size-4 bg-secondary" />
                    </div>
                    <div class="flex items-end gap-2">
                        <span class="text-4xl font-roboto leading-none">{pipeline.successRate}%</span>
                        <span class="text-[11px] uppercase tracking-wider opacity-45 pb-1">successfully embedded</span>
                    </div>
                    <div class="h-2 w-full rounded-full bg-primary/5 overflow-hidden flex">
                        <div class="h-full bg-secondary-1" style="width: {pipeline.donePercent}%"></div>
                        <div class="h-full bg-amber-400" style="width: {pipeline.pendingPercent}%"></div>
                        <div class="h-full bg-rose-400" style="width: {pipeline.failedPercent}%"></div>
                    </div>
                    <div class="grid grid-cols-3 gap-2 text-xs">
                        <div class="flex flex-col gap-0.5">
                            <span class="font-mono text-secondary-1">{formatNumber(pipeline.done)}</span>
                            <span class="opacity-40 uppercase tracking-wider text-[10px]">done</span>
                        </div>
                        <div class="flex flex-col gap-0.5">
                            <span class="font-mono text-amber-500">{formatNumber(pipeline.pending)}</span>
                            <span class="opacity-40 uppercase tracking-wider text-[10px]">pending</span>
                        </div>
                        <div class="flex flex-col gap-0.5">
                            <span class="font-mono text-rose-500">{formatNumber(pipeline.failed)}</span>
                            <span class="opacity-40 uppercase tracking-wider text-[10px]">failed</span>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Breakdowns -->
            <section class="grid gap-4 xl:grid-cols-3">
                <div class="glass p-5 flex flex-col gap-4">
                    {@render tableHeader("File types", `${statistics.file_types.length} types`, "/box.svg")}
                    {#if statistics.file_types.length === 0}
                        <p class="text-sm italic opacity-40">No files have been uploaded yet.</p>
                    {:else}
                        <div class="flex flex-col gap-3">
                            {#each statistics.file_types as type}
                                <div class="flex flex-col gap-1">
                                    <div class="flex items-baseline justify-between text-sm gap-3">
                                        <span class="font-mono uppercase tracking-wide truncate">{type.file_type}</span>
                                        <span class="opacity-50 text-xs whitespace-nowrap">{formatNumber(type.count)} · {percent(type.percentage)}</span>
                                    </div>
                                    <div class="h-1.5 rounded-full bg-primary/5 overflow-hidden">
                                        <div class="h-full rounded-full bg-gradient-to-r from-secondary to-secondary-1" style="width: {type.percentage}%"></div>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <div class="glass p-5 flex flex-col gap-4">
                    {@render tableHeader("Top tags", "most applied", "/archive.svg")}
                    {#if statistics.top_tags.length === 0}
                        <p class="text-sm italic opacity-40">No tags have been assigned yet.</p>
                    {:else}
                        <div class="flex flex-col divide-y divide-primary/5">
                            {#each statistics.top_tags as tag, i}
                                <div class="flex items-center justify-between gap-3 py-2 text-sm">
                                    <div class="flex items-center gap-3 min-w-0">
                                        <span class="font-mono text-[11px] opacity-30 w-5 shrink-0">{i + 1}</span>
                                        <span class="glass px-3 py-0.5 rounded-full bg-secondary/5 text-secondary border-secondary/40 truncate">{tag.tag}</span>
                                    </div>
                                    <span class="font-mono text-xs opacity-50 shrink-0">{formatNumber(tag.count)}</span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <div class="glass p-5 flex flex-col gap-4">
                    {@render tableHeader("Monthly activity", `${formatNumber(monthly.total)} in 6 months`, "/best.svg")}
                    <div class="flex flex-col gap-2">
                        {#each monthly.rows as row}
                            {@const width = monthly.max > 0 ? (row.count / monthly.max) * 100 : 0}
                            <div class="relative overflow-hidden rounded-lg">
                                <div class="absolute inset-y-0 left-0 bg-secondary/10" style="width: {width}%"></div>
                                <div class="relative flex items-center justify-between gap-3 px-3 py-2 text-sm">
                                    <span class="truncate">{monthLabel(row.month)}</span>
                                    <span class="font-mono text-xs opacity-60 shrink-0">{formatNumber(row.count)}</span>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            </section>

            <!-- Recent uploads -->
            <section class="glass p-5 flex flex-col gap-4">
                {@render tableHeader("Recent uploads", `latest ${statistics.recent_uploads.length}`, "/inbox.svg")}
                {#if statistics.recent_uploads.length === 0}
                    <p class="text-sm italic opacity-40">Nothing has been uploaded yet.</p>
                {:else}
                    <div class="overflow-x-auto">
                        <div class="min-w-[680px]">
                            <div class="grid grid-cols-[minmax(0,1fr)_90px_180px_105px_120px] gap-3 px-3 pb-2 text-[11px] font-mono uppercase tracking-wider opacity-40">
                                <span>File</span>
                                <span>Type</span>
                                <span>Contributor</span>
                                <span>Status</span>
                                <span class="text-right">Date</span>
                            </div>
                            <div class="flex flex-col divide-y divide-primary/5">
                                {#each statistics.recent_uploads as upload}
                                    <div class="grid grid-cols-[minmax(0,1fr)_90px_180px_105px_120px] gap-3 px-3 py-2.5 text-sm items-center rounded-lg transition-colors hover:bg-secondary/5">
                                        <span class="truncate" title={upload.name}>{upload.name}</span>
                                        <span class="glass w-min px-2 py-0.5 rounded-full text-[10px] font-mono uppercase text-amber-600 border-amber-200 bg-amber-100/40">
                                            {upload.file_type}
                                        </span>
                                        <span class="truncate opacity-60" title={upload.submitted_by ?? undefined}>#{upload.submitted_by ?? "unknown"}</span>
                                        <span class="glass w-min px-2 py-0.5 rounded-full text-[10px] font-mono uppercase border {statusStyles[upload.status] ?? 'bg-primary/5 text-primary border-primary/10'}">
                                            {upload.status}
                                        </span>
                                        <span class="text-right font-mono text-xs opacity-50">{dateLabel(upload.date)}</span>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    </div>
                {/if}
            </section>
        {/if}
    </div>
</div>
