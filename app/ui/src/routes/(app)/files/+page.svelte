<script lang="ts">
import { page } from "$app/state";

import { invalidate, pushState } from "$app/navigation";
import { client } from "$lib/api";
import Fab from "$lib/components/Fab.svelte";
import FilePreview from "$lib/components/FilePreview.svelte";
import FilesGrid from "$lib/components/FilesGrid.svelte";
import FilesList from "$lib/components/FilesList.svelte";
import { reportError } from "$lib/components/Toast.svelte";
import type { SchemaFileData } from "$lib/openapi";
import { uploadFile } from "$lib/upload";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { Localized, getFluentContext } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

let { data }: Props = $props();

let files = $state(data.files);

$effect(() => {
	files = data.files;
});

let searchQuery = $state(data.search);
let queryError = $state(data.queryError);

let uploadInput: HTMLInputElement;

type ViewMode = "grid" | "list";
let view: ViewMode = $state("list");

const FilesView = $derived(view === "list" ? FilesList : FilesGrid);

const { localize } = getFluentContext();

function togglePreview(preview?: SchemaFileData) {
	pushState("", { preview });
}

function pickFile() {
	if (!uploadInput) {
		console.warn("uploadInput reference was missing:", uploadInput);
		return;
	}

	uploadInput.click();
}

async function upload(ev: Event) {
	const input = ev.target as HTMLInputElement;
	for (const file of input.files || []) {
		await uploadFile(file, localize);
	}

	await invalidate("/api/v1/files");
}

async function deleteFile(file: SchemaFileData) {
	const { error } = await client.DELETE("/api/v1/files/{file_id}", {
		params: { path: { file_id: file.id } },
	});

	if (error) {
		reportError(error);
		return;
	}

	await invalidate(
		(url) =>
			url.pathname === "/api/v1/files" ||
			url.pathname === `/api/v1/files/${file.id}`,
	);
}

let debounceSearchTimer: ReturnType<typeof setTimeout>;
async function search(ev: Event) {
	queryError = "";
	const input = ev.target as HTMLInputElement;

	clearTimeout(debounceSearchTimer);
	debounceSearchTimer = setTimeout(async () => {
		const search = input.value;
		searchQuery = search;
		pushState(`?search=${search}`, {});

		const { data, error } = await client.GET("/api/v1/files", {
			params: {
				query: {
					search,
				},
			},
		});

		if (error) {
			switch (error.error) {
				case "INVALID_QUERY":
					queryError = error.message;
					break;
				default:
					reportError(error);
			}
			return;
		}

		// biome-ignore lint/style/noNonNullAssertion: we checked that error is absent, so data cannot also be absent
		files = data!;
	}, 100);
}
</script>

<h1 class="sr-only"><Localized id="files-title" /></h1>

<div class="toolbar">
    <div class="toolbar-group">
        <input class="checkbox" type="checkbox" />
    </div>

    <div class="search">
        <Localized id="files-search">
            {#snippet children({ text, attrs })}
                <input type="search"
                       class="input"
                       onkeyup={search}
                       placeholder={text}
                       aria-label={attrs.label}
                       value={searchQuery}
                       autocomplete="off" />
            {/snippet}
        </Localized>
    </div>

    <div class="toolbar-group">
        <Localized id="files-switch-view">
            {#snippet children({ text, attrs })}
                {#if view == "list"}
                    <button aria-label={attrs['to-grid']} onclick={() => view = 'grid'}>
                        <i class="fa-solid fa-table-cells-large"></i>
                    </button>
                {:else}
                    <button aria-label={attrs['to-list']} onclick={() => view = 'list'}>
                        <i class="fa-solid fa-list-ul"></i>
                    </button>
                {/if}
            {/snippet}
        </Localized>
    </div>
</div>

{#if queryError}<p class="error text-danger-500 text-xs">{queryError}</p>{/if}

{#if files.items.length > 0}
<FilesView
    {files}
    ondelete={deleteFile}
    onpreview={togglePreview}
/>
{:else}
    <div class="empty">
        <p><Localized id="files-empty" /></p>
    </div>
{/if}

{#if page.state.preview}
    <FilePreview file={page.state.preview} onclose={() => togglePreview()} />
{/if}

<Fab color="primary" icon={faPlus} label="upload" onclick={pickFile}>
    <input
        bind:this={uploadInput}
        type="file"
        multiple
        onchange={upload}
        hidden
    />
</Fab>

<style>
 .toolbar {
     display: flex;
     flex-direction: row;
     justify-content: space-between;
     align-items: center;
     padding: var(--oxi-size-sm);
     gap: var(--oxi-size-sm);
 }

 .toolbar-group {
     display: flex;
     flex-direction: row;
     justify-content: flex-start;
     align-items: center;
 }

.search {
    flex: 2 1 0;
}

 .error {
     padding: var(--oxi-size-sm);
 }

.empty {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>
