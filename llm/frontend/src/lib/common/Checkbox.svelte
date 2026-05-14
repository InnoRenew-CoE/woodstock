<script lang="ts">
    let { group = $bindable([]), class: className = "", label, value, multiple = true } = $props();

    const appropriateType = () => (typeof value === "number" ? +value : value);
    let checked = $derived(group.includes(appropriateType()));

    function change(e: Event) {
        const value = appropriateType();
        if (!checked) {
            if (!group.includes(value)) {
                if (multiple) {
                    group = [...group, value];
                } else {
                    group = [value];
                }
            }
        } else {
            group = group.filter((a) => a !== value);
        }
    }
</script>

<label class="transition-all select-none glass px-5 py-3 {checked ? 'bg-secondary/10 text-secondary border-secondary hover:bg-red-100 hover:text-red-400 hover:border-red-400 ' : 'hover:bg-secondary/10 hover:border-secondary hover:text-accent'}">
    <input class="hidden" type="checkbox" {value} onchange={(e) => change(e)} />
    {label}
</label>
