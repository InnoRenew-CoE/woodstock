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

<label class="transition-all font-nunito select-none cursor-pointer px-4 py-2 rounded border {checked ? 'bg-accent/70 border-accent text-white' : 'border-gray-200 bg-gray-100 opacity-70 hover:opacity-100'} ">
    <input class="hidden" type="checkbox" {value} onchange={(e) => change(e)} />
    {label}
</label>
