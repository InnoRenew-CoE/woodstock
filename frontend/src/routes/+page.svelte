<script>
    import { goto } from "$app/navigation";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import TextInput from "$lib/inputs/TextInput.svelte";
    import { pushNotification } from "$lib/stores/notifications";

    let email = $state("");
    let password = $state("");

    async function login() {
        if (email.length > 5 && password.length > 5) {
            pushNotification({ body: "Succesfull login", title: "Welcome" });
            await goto("/app");
        }
    }
</script>

<div class="flex justify-center h-full p-10">
    <div class="grid gap-2 max-w-[400px] w-[80%] h-min">
        <form class="grid p-5 px-8 rounded-xl border gap-5 bg-dark-background">
            <div class="flex gap-3 items-center text-lg justify-center">
                WoodStock <span><img src="./info.svg" alt="Information" class="w-3" /></span>
            </div>
            <TextInput required placeholder="Organisation e-mail" type="email" bind:value={email} icon="../email.svg" />
            <TextInput required placeholder="Password" type="password" bind:value={password} icon="../password.svg" />
            <button type="submit" class="primary-button grid grid-cols-[1rem_auto_1rem] items-center gap-4 group" onclick={login}>
                <MaskedIcon src="../login.svg" class="size-4 group-hover:translate-x-full bg-white transition duration-200 ease-in-out" />
                <div class="row-start-1 col-start-2">Login</div>
            </button>
        </form>
    </div>
</div>
