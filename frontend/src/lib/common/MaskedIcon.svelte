<script lang="ts">
    let { src = undefined, class: className = "" } = $props();
</script>

<div style="mask-image: url({src});" class="w-4 h-4 bg-primary {className}"></div>

<style>
    div {
        mask-repeat: no-repeat;
        mask-position: center;
    }
</style>
