<script>
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import TextInput from "$lib/inputs/TextInput.svelte";

    let email = "";
</script>

<div class="flex justify-center items-center rounded-lg h-full p-10">
    <div class="grid gap-5 max-w-[400px] w-[80%]">
        <div class="flex justify-center items-center">
            <img src="./woodstock.svg" alt="Logo" class="rounded-full w-14" />
        </div>
        <form class="grid p-5 px-8 rounded-xl border gap-5 bg-dark-background">
            <div class="flex gap-3 items-center text-lg justify-center">
                WoodStock <span><img src="./info.svg" alt="Information" class="w-3" /></span>
            </div>
            <TextInput placeholder="Organisation e-mail" type="email" bind:value={email} icon="../email.svg" />
            <TextInput placeholder="Password" type="password" icon="../password.svg" />
            <button class="primary-button grid grid-cols-[1rem_auto_1rem] items-center gap-4 group">
                <MaskedIcon src="../login.svg" class="size-4 group-hover:translate-x-full bg-white transition duration-200 ease-in-out" />
                <div class="row-start-1 col-start-2">Login</div>
            </button>
        </form>
    </div>
</div>
