<script lang="ts">
import type { SchemaTag } from "$lib/openapi.ts";
import { concat, split } from "$lib/tags";
import type { Snippet } from "svelte";

interface Props {
	tag: SchemaTag | string;
	size?: "small" | "regular";
	class?: string;
	action?: Snippet;
}

let { tag: t, size = "regular", class: clazz = "", action }: Props = $props();

const label = concat(t);
const tag = split(t);
</script>

<span class={`tag ${clazz}`} class:small={size === "small"} title={label}>
    <span class="key"
        >{tag.key}{#if tag.value}:{/if}</span
    >
    {#if tag.value}
        <span class="value">{tag.value}</span>
    {/if}

    {@render action?.()}
</span>

<style lang="scss">
  @use "$lib/styles/oxi";

</style>
