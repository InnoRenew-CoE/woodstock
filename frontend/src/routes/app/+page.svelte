<script lang="ts">
    const colors = ["bg-accent", "bg-secondary", "bg-primary", "bg-orange-400", "bg-red-300"];
</script>

<div class="grid gap-5 grid-cols-3 grid-rows-3 h-full">
    {#each new Array(9) as _}
        <div class="text-white rounded-lg {colors[Math.floor(Math.random() * colors.length)]} self-stretch flex items-center justify-center">Sexy Charts</div>
    {/each}
</div>
