<script lang="ts">
import { pushState } from "$app/navigation";
import { page } from "$app/state";
import FilePreview from "$lib/components/FilePreview.svelte";
import FilesGrid from "$lib/components/FilesGrid.svelte";
import FilesList from "$lib/components/FilesList.svelte";
import Filter from "$lib/components/Filter.svelte";
import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
import type { SchemaFileData } from "$lib/openapi";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

const { data }: Props = $props();

type ViewMode = "grid" | "list";
let view: ViewMode = $state("list");

const FilesView = $derived(view === "list" ? FilesList : FilesGrid);

function togglePreview(preview?: SchemaFileData) {
	pushState("", { preview });
}
</script>

<header class="header">
    <h1 class="title">{data.collection.name}</h1>
    <Filter filter={data.collection.filter} />
</header>

{#await data.lazy.files}
    <div class="loading">
        <LoadingSpinner />
    </div>
{:then files}
    {#if files.items.length > 0}
        <FilesView
            {files}
            ondelete={() => {}}
            onpreview={(file) => togglePreview(file)}
        />
    {:else}
        <div class="empty">
            <p><Localized id="files-empty" /></p>
        </div>
    {/if}

    {#if page.state.preview}
        <FilePreview
            file={page.state.preview}
            onclose={() => togglePreview()}
        />
    {/if}
{/await}

<style>
    .header {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: var(--oxi-size-m);
        width: 100%;
        gap: var(--oxi-size-xs);
    }

    .loading {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: var(--oxi-size-m);
        width: 100%;
    }

    .empty {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

 :global(.filter-container) {
     text-align: center;
     width: 100%;
}
</style>
