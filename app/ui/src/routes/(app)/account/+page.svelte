<script lang="ts">
import { client } from "$lib/api";
import Button from "$lib/components/Button.svelte";
import { notify, reportError } from "$lib/components/Toast.svelte";
import { Localized, getFluentContext } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

const { data }: Props = $props();

const { localize } = getFluentContext();

let loading = $state(false);

let currentPassword = $state("");
let currentPasswordError = $state("");

let newPassword = $state("");
let newPasswordError = $state("");

async function changePassword(ev: SubmitEvent) {
	ev.preventDefault();
	loading = true;

	currentPasswordError = "";
	newPasswordError = "";

	const { error } = await client.PUT("/api/v1/accounts/me/password", {
		body: { current_password: currentPassword, new_password: newPassword },
	});

	if (error) {
		switch (error.error) {
			case "INVALID_CURRENT_PASSWORD": {
				currentPasswordError = localize(
					"account-settings-current-password.error-invalid",
				);
				break;
			}
			case "INVALID_NEW_PASSWORD": {
				newPasswordError = localize(
					"account-settings-new-password.error-invalid",
				);
				break;
			}
			default:
				reportError(error);
		}
		loading = false;
		return;
	}

	notify({
		title: localize("account-settings-password-changed"),
		message: localize("account-settings-password-changed.message"),
	});
	currentPassword = "";
	newPassword = "";
	loading = false;
}
</script>

<div class="account">
    <h1 class="title"><Localized id="account-settings-title" /></h1>

    <section class="section">
        <h2 class="subtitle"><Localized id="account-settings-details" /></h2>

        <span>
            <label for="id">
                <Localized id="account-settings-id" />
            </label>
            <input
                class="input"
                value={data.account.id}
                name="id"
                onclick={(ev) => ev.currentTarget.select()}
                readonly
            />
        </span>

        <span>
            <label for="id">
                <Localized id="account-settings-username" />
            </label>
            <input
                class="input"
                value={data.account.username}
                name="username"
                onclick={(ev) => ev.currentTarget.select()}
                readonly
            />
        </span>
    </section>

    <section class="section">
        <h2 class="subtitle"><Localized id="account-settings-password" /></h2>

        <form class="form" onsubmit={changePassword}>
            <div class="fieldset inputs">
                <div class="field">
                <Localized id="account-settings-current-password">
                    {#snippet children({ text })}
                        <label for="current-password" class="sr-only">{text}</label>
                        <input
                            class="input"
                            class:error={!!currentPasswordError}
                            type="password"
                            placeholder={text}
                            name="current-password"
                            bind:value={currentPassword}
                        />
                        {#if currentPasswordError}<p class="error">{currentPasswordError}</p>{/if}
                    {/snippet}
                </Localized>
                </div>

                <div class="field">
                <Localized id="account-settings-new-password">
                    {#snippet children({ text })}
                        <label for="new-password" class="sr-only">{text}</label>
                        <input
                            class="input"
                            class:error={!!newPasswordError}
                            type="password"
                            placeholder={text}
                            name="new-password"
                            bind:value={newPassword}
                        />
                        {#if newPasswordError}<p class="error">{newPasswordError}</p>{/if}
                    {/snippet}
                </Localized>
                </div>
            </div>

            <div class="fieldset buttons">
                <Button type="submit" variant="filled" {loading}
                    ><Localized id="account-settings-change-password" />
                </Button>
            </div>
        </form>
    </section>
</div>

<style lang="scss">
 @use "$lib/styles/oxi";

 .account {
     display: flex;
     flex-direction: column;
     gap: oxi.size(m);
     padding: oxi.size(sm);
 }

    .section {
        display: flex;
        flex-direction: column;
        gap: oxi.size(sm);
    }
</style>
