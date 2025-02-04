<script lang="ts">
import "$lib/styles/styles.scss";

import {
	createSvelteFluent,
	initFluentContext,
} from "@nubolab-ffwd/svelte-fluent";

import { afterNavigate, beforeNavigate } from "$app/navigation";
import { setUser } from "$lib/auth";
import Loading from "$lib/components/Loading.svelte";
import Toast from "$lib/components/Toast.svelte";
import { generateBundles } from "$lib/languages";
import type { LayoutData } from "./$types";

interface Props {
	data: LayoutData;
	children?: import("svelte").Snippet;
}

let { data, children }: Props = $props();

setUser(data.currentUser);

let load = $state(false);

initFluentContext(() =>
	createSvelteFluent(
		generateBundles(data.translations.resources, data.translations.languages),
	),
);

beforeNavigate(() => {
	load = true;
});

afterNavigate(() => {
	load = false;
});
</script>

<Toast />

{#if load}
    <Loading />
{:else}
    {@render children?.()}
{/if}
