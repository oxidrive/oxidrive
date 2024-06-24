<script lang="ts">
import type { File } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";

export let file: File;

let clazz = "";
export { clazz as class };
</script>

{#if file.type === "folder"}
    <Localized id="files-link-open-folder" let:text args={{ file: file.name }}>
        <a class={clazz} href={file.path} title={text}><slot /></a>
    </Localized>
{:else}
        <a
            class={clazz}
            href="{window.location.pathname}?preview={file.name}"
            title={file.name}
        >
            <slot />
        </a>
{/if}
