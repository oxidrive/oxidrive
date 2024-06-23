<script lang="ts">
import { goto } from "$app/navigation";
import { client } from "$lib/api";
import Logo from "$lib/components/Logo.svelte";
import PageTitle from "$lib/components/PageTitle.svelte";
import { notify } from "$lib/components/Toast.svelte";
import { Localized, localize } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

export let data: PageData;

let setupError = "";
const recapEntries = [
	{ name: "database", value: data.status.database },
	{ name: "file-storage", value: data.status.fileStorage },
];

let username = "";
let password = "";
let passwordConfirm = "";

function checkPasswords(): boolean {
	if (setupError) return true;

	if (password && passwordConfirm && password !== passwordConfirm) {
		setupError = "setup-form-password-mismatch";
		return false;
	}

	return true;
}

async function submit() {
	if (!checkPasswords()) {
		return;
	}

	const { error } = await client.POST("/api/instance/setup", {
		body: {
			admin: { username, password },
		},
	});

	if (error) {
		reportError(error);
		return;
	}

	notify({
		title: $localize("setup-succeeded"),
		message: $localize("setup-succeeded.message"),
	});
	return goto("/");
}
</script>

<picture class="logo">
    <Logo color="primary" name />
</picture>

<Localized id="setup-title" let:text>
    <PageTitle title={text} />
    <h1 class="text-primary-500 text-lg">{text}</h1>
</Localized>

{#if setupError}
    <Localized id={setupError} let:text>
        <p class="text-danger-500 text-center">{text}</p>
    </Localized>
{/if}

<form class="form" method="POST" on:submit|preventDefault={submit}>
    <div class="fieldset inputs">
        <Localized id="setup-form-username" let:text>
            <label for="username" class="label">{text}</label>
            <input
                class="input"
                class:error={setupError}
                name="username"
                placeholder={text}
                bind:value={username}
                on:input={() => (setupError = "")}
            />
        </Localized>

        <Localized id="login-form-password" let:text>
            <label for="password" class="label">{text}</label>
            <input
                class="input"
                class:error={setupError}
                type="password"
                name="password"
                placeholder={text}
                bind:value={password}
                on:input={() => (setupError = "")}
                on:blur={checkPasswords}
            />
        </Localized>

        <Localized id="setup-form-confirm-password" let:text>
            <label for="password-confirmation" class="label">{text}</label>
            <input
                class="input"
                class:error={setupError}
                type="password"
                name="password-confirmation"
                placeholder={text}
                bind:value={passwordConfirm}
                on:input={() => (setupError = "")}
                on:blur={checkPasswords}
            />
        </Localized>
    </div>

    <div class="recap-box">
        <Localized id="setup-form-configuration-recap" let:text>
            <h2 class="title text-primary-600 text-bold">{text}</h2>
        </Localized>

        <a class="recap-link" href={data.status.publicURL}>
            <h3 class="title">{data.status.publicURL}</h3>
        </a>

        <div class="recap-entries">
            {#each recapEntries as { name, value }}
                <Localized id="setup-form-configuration-recap" let:attrs>
                    <p class="recap-entry">
                        <span class="text-bold">{attrs[name]}:</span>
                        <span class="truncate">{value}</span>
                    </p>
                </Localized>
            {/each}
        </div>
    </div>

    <div class="fieldset buttons">
        <Localized id="setup-form-submit-cta" let:text>
            <input class="button filled" type="submit" value={text} />
        </Localized>

        <Localized id="setup-form-help-cta" let:text>
            <a
                class="button ghost"
                href="https://github.com/oxidrive/oxidrive/discussions"
                target="_blank"
            >
                {text}
            </a>
        </Localized>
    </div>
</form>

<style lang="scss">
    .logo {
        height: 100px;
        width: 100px;
    }

    .recap-box {
        background-color: var(--oxi-color-primary-200);
        border-radius: var(--oxi-rounded-3xl);
        padding: var(--oxi-size-2);
        display: inline-block;
        max-width: 90%;

        .title {
            text-align: center;
        }

        .recap-link {
            color: var(--oxi-color-primary-600);
            text-decoration: underline;
        }

        .recap-entries {
            padding: var(--oxi-size-2);
        }

        .recap-entry {
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            gap: var(--oxi-size-4);
            align-content: space-evenly;
            align-items: start;
            justify-content: space-between;
            color: var(--oxi-color-primary-600);
            white-space: nowrap;
        }
    }
</style>
