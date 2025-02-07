<script lang="ts">
import logo from "$lib/assets/images/logo-with-name.svg";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

let { data }: Props = $props();
</script>

<picture class="logo">
    <img class="logo" src={logo} alt="Oxidrive" />
</picture>

<h1><Localized id="login-title" /></h1>

{#if data.error}
    <p>{data.error}</p>
{/if}

<form class="form" action="/auth/session?redirect_to={data.redirect_to}" method="post">
    <div class="fieldset inputs">
        <Localized id="login-form-username" >
            {#snippet children({ text })}
                <label for="username" class="sr-only">{text}</label>
                <input class="input" type="text" name="username" placeholder={text} />
            {/snippet}
        </Localized>

        <Localized id="login-form-password" >
            {#snippet children({ text })}
                <label for="password" class="sr-only">{text}</label>
                <input class="input" type="password" name="password" placeholder="Password" />
            {/snippet}
        </Localized>

        <input type="hidden" name="kind" value="password" />
    </div>


    <div class="fieldset buttons">
        <Localized id="login-form-submit-cta">
            {#snippet children({ text })}
                <button class="button filled" type="submit">{text}</button>
            {/snippet}
        </Localized>
    </div>
</form>

<style>
    .logo {
        height: 100px;
        width: 100px;
    }
</style>
