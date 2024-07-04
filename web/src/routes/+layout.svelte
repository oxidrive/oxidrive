<script lang="ts">
import "../styles/styles.scss";

import { initUser } from "$lib/auth";
import Toast from "$lib/components/Toast.svelte";
import { FluentProvider } from "@nubolab-ffwd/svelte-fluent";

import { generateBundles } from "$lib/languages";
import type { LayoutData } from "./$types";

export let data: LayoutData;

initUser(data.session?.user);
</script>

<FluentProvider bundles={generateBundles(data.resources, data.languages)}>
    <Toast />
    <slot />
</FluentProvider>
