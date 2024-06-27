<script lang="ts">
import type { File } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher<{ preview: File }>();

export let file: File;

let clazz = "";
export { clazz as class };
</script>

{#if file.type === "folder"}
    <Localized id="files-link-open-folder" let:text args={{ file: file.name }}>
        <a class={clazz} href="/files{file.path}" title={text}><slot /></a>
    </Localized>
{:else}
    <button
        class={clazz}
        on:click={() => dispatch("preview", file)}
        title={file.name}
    >
        <slot />
    </button>
{/if}
