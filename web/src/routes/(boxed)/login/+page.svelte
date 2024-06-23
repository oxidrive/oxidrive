<script lang="ts">
import { goto } from "$app/navigation";
import { client } from "$lib/api";
import { getUser } from "$lib/auth";
import Logo from "$lib/components/Logo.svelte";
import PageTitle from "$lib/components/PageTitle.svelte";
import { addToast, reportError } from "$lib/components/Toast.svelte";
import { Localized, localize } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

export let data: PageData;

let username = "";
let password = "";

let authFailed = false;

const user = getUser();

async function submit() {
	const { data: body, error } = await client.POST("/api/sessions", {
		body: {
			credentials: {
				kind: "password",
				username,
				password,
			},
		},
	});

	if (error && error.error === "authentication_failed") {
		authFailed = true;
		return;
	}

	if (error) return reportError(error);
	if (!body) throw new Error("API returned no error, but no data either");

	addToast({
		data: {
			title: $localize("login-auth-succeeded"),
			message: $localize("login-auth-succeeded.message"),
			level: "info",
		},
	});
	user.set(body.user);
	goto(data.to, { invalidateAll: true });
}
</script>

<picture class="logo">
    <Logo color="primary" name />
</picture>

<Localized id="login-title" let:text>
    <PageTitle title={text} />
    <h1 class="text-primary-500 text-lg">{text}</h1>
</Localized>

{#if authFailed}
    <Localized id="login-auth-failed" let:attrs>
        <p class="text-danger-500">{attrs.message}</p>
    </Localized>
{/if}
<form class="form" method="POST" on:submit|preventDefault={submit}>
    <div class="fieldset inputs">
        <Localized id="login-form-username" let:text>
            <label for="username" class="label">{text}</label>
            <input
                class="input"
                class:error={authFailed}
                name="username"
                placeholder={text}
                bind:value={username}
                on:input={() => (authFailed = false)}
            />
        </Localized>

        <Localized id="login-form-password" let:text>
            <label for="password" class="label">{text}</label>
            <input
                class="input"
                class:error={authFailed}
                type="password"
                name="password"
                placeholder={text}
                bind:value={password}
                on:input={() => (authFailed = false)}
            />
        </Localized>
    </div>

    <div class="fieldset buttons">
        <Localized id="login-form-submit-cta" let:text>
            <input class="button filled" type="submit" value={text} />
        </Localized>
    </div>
</form>

<style>
    .logo {
        height: 100px;
        width: 100px;
    }
</style>
