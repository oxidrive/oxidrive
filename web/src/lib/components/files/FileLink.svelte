<script lang="ts">
import type { File } from "$lib/api";

export let file: File;
</script>

{#if file.type === "folder"}
    <a href={`/files${file.path}`}><slot /></a>
{:else}
    <slot />
{/if}
