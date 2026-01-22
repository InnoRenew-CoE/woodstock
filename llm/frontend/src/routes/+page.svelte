<script lang="ts">
    import { goto } from "$app/navigation";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { verify } from "$lib";
    import CheckMarkCheckbox from "$lib/common/CheckMarkCheckbox.svelte";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import SuggestiveInput from "$lib/common/SuggestiveInput.svelte";
    import TextInput from "$lib/inputs/TextInput.svelte";
    import { pushNotification } from "$lib/stores/notifications";
    import { fetchQuestions } from "$lib/stores/questions";
    import { onMount } from "svelte";
    import { passive } from "svelte/legacy";
    import { fade } from "svelte/transition";

    let windowSize = $state(0);
    let email = $state("");
    let password = $state("");
    let requestPassword = $state(false);

    async function login() {
        const init: RequestInit = {
            method: "post",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ email: email, password: password }),
        };
        if (requestPassword) {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/register`, init);
            if (response.status === 200) {
                pushNotification({ body: "You'll receive an email with newly generated password.", title: "Request successful" });
                requestPassword = false;
            } else {
                pushNotification({ body: "You're not on our whitelist. Please notify us at ewco@innorenew.eu", title: "Request failed" });
            }
        } else {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/login`, init);
            if (response.status === 200) {
                pushNotification({ body: "Succesfull login", title: "Welcome" });
                await goto("/app");
                console.log(response.headers.getSetCookie());
            } else {
                pushNotification({ body: "Incorrect login credentials", title: "Failure to login" });
            }
        }
    }
    onMount(async () => {
        if ((await verify()) === 200) {
            await goto("/app");
        }
    });
</script>

<svelte:window bind:innerWidth={windowSize} />

<div class="flex sm:items-center justify-center h-full p-10">
    <div class="grid gap-2 max-w-[500px] w-full sm:w-[80%] h-min">
        <div class="grid p-8 px-10 glass gap-5 bg-white/60">
            <div class="flex gap-3 items-center font-roboto font-semibold text-2xl text-black">
                Login <span><img src="./info.svg" alt="Information" class="w-3" /></span>
            </div>
            <div>
                <div class="text-sm pb-1 font-nunito text-primary/30">E-mail</div>
                <TextInput required placeholder="you@organisation.eu" type="email" bind:value={email} icon="../email.svg" />
            </div>
            <CheckMarkCheckbox label="Request access" bind:checked={requestPassword} />
            {#if !requestPassword}
                <div in:fade>
                    <TextInput required placeholder="*****" type="password" bind:value={password} icon="../password.svg" />
                </div>
            {:else}
                <div in:fade>
                    <p class="bg-white/70 p-3 rounded-lg shadow-sm border border-white text-secondary/80">You'll receive an email with password for you to access the app.</p>
                </div>
            {/if}
            <button type="submit" class="transition-all hover:bg-secondary/80 hover:border-secondary cursor-pointer bg-primary border border-accent/80 rounded-lg p-2 text-white grid grid-cols-[1rem_auto_1rem] items-center gap-4 group" onclick={login}>
                <div class="row-start-1 col-start-2">{requestPassword ? "Request" : "Login"}</div>
            </button>
        </div>
    </div>
</div>
