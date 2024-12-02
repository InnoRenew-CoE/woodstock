<script lang="ts">
    import "../app.css";
    let { children } = $props();
</script>

<div id="layout">
    <div>Header</div>

    <div class="bg-red-300 p-5 min-h-full">
        {@render children()}
    </div>

    <footer>foot</footer>
</div>

<style>
    #layout {
        /*
        Calculation is the following: 2rem header, 100vh - (footer + header) for content.
        This way the content is either full screen or more.
        */
        display: grid;
        grid-template-rows:
            2rem minmax(calc(100vh - calc(100px + 2rem)), auto)
            100px;
    }
</style>
