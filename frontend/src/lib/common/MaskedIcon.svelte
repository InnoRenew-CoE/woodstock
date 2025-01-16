<script lang="ts">
    let { src = undefined, class: className = "w-4 h-4 bg-primary" } = $props();
</script>

<div style="mask-image: url({src});" class={className}></div>

<style>
    div {
        mask-repeat: no-repeat;
        mask-position: center;
    }
</style>
