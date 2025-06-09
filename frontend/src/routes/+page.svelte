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
                pushNotification({ body: "An error occured, please notify us at woodstock@innorenew.eu", title: "Request failed" });
            }
        } else {
            const response = await fetch(`${PUBLIC_API_BASE_URL}/login`, init);
            if (response.status === 200) {
                pushNotification({ body: "Succesfull login", title: "Welcome" });
                await goto("/app");
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

<div class="flex justify-center h-full p-10">
    <div class="grid gap-2 max-w-[400px] w-[80%] h-min">
        <form class="grid p-5 px-8 rounded-xl border border-accent shadow-lg gap-5 bg-accent/[2%]">
            <div class="flex gap-3 items-center text-lg justify-center font-mono text-accent">
                Login <span><img src="./info.svg" alt="Information" class="w-3" /></span>
            </div>
            <TextInput required placeholder="Organisation e-mail" type="email" bind:value={email} icon="../email.svg" />
            <CheckMarkCheckbox label="Request access" bind:checked={requestPassword} />
            {#if !requestPassword}
                <div in:fade>
                    <TextInput required placeholder="Password" type="password" bind:value={password} icon="../password.svg" />
                </div>
            {:else}
                <div in:fade>
                    <p class="bg-secondary/5 border border-secondary p-3 rounded text-sm">You'll receive an email with password for you to access the app.</p>
                </div>
            {/if}
            <button type="submit" class="transition-all hover:bg-primary cursor-pointer bg-accent/70 border border-accent/80 rounded-md p-2 text-white grid grid-cols-[1rem_auto_1rem] items-center gap-4 group" onclick={login}>
                <div class="row-start-1 col-start-2">{requestPassword ? "Request" : "Login"}</div>
            </button>
        </form>
    </div>
</div>
