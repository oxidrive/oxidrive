<script lang="ts">
import { invalidate, pushState } from "$app/navigation";
import { page } from "$app/stores";
import { type ErrorResponse, type File, type FileList, client } from "$lib/api";
import Loading from "$lib/components/Loading.svelte";
import PageTitle from "$lib/components/PageTitle.svelte";
import { addToast, reportError } from "$lib/components/Toast.svelte";
import Fab from "$lib/components/buttons/Fab.svelte";
import FilePreview from "$lib/components/files/FilePreview.svelte";
import FilesGrid from "$lib/components/files/FilesGrid.svelte";
import FilesList from "$lib/components/files/FilesList.svelte";
import NoFiles from "$lib/components/files/NoFiles.svelte";
import { rename as renamePath } from "$lib/paths";
import { faFile, faFolder, faPlus } from "@fortawesome/free-solid-svg-icons";
import { Localized, localize } from "@nubolab-ffwd/svelte-fluent";
import { get, writable } from "svelte/store";
import type { PageData } from "./$types";

const NEW_FOLDER_ID = "00000000-0000-0000-0000-000000000000";

let viewMode: "list" | "grid" = "grid";
let viewComponent = FilesGrid;

export let data: PageData;
let files: FileList | null = null;

const selected = writable(new Set<File>());

let uploadInput: HTMLInputElement;

function setFiles(list: FileList | null): "" {
	files = list;
	return "";
}

function switchMode() {
	switch (viewMode) {
		case "grid": {
			viewMode = "list";
			viewComponent = FilesList;
			break;
		}
		case "list": {
			viewMode = "grid";
			viewComponent = FilesGrid;
			break;
		}
	}
}

function pickFile() {
	if (!uploadInput) {
		console.warn("uploadInput reference was missing:", uploadInput);
		return;
	}

	uploadInput.click();
}

async function select({ detail: file }: CustomEvent<File>) {
	selected.update((selected) => selected.add(file));
}

async function deselect({ detail: file }: CustomEvent<File>) {
	selected.update((selected) => {
		selected.delete(file);
		return selected;
	});
}

async function escapeJSONFile(file: globalThis.File): Promise<globalThis.File> {
	const json = await new Promise<string>((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsText(file, "UTF-8");
		reader.addEventListener("load", () => {
			resolve(reader.result as string);
		});
		reader.addEventListener("error", reject);
	});

	return new globalThis.File([json], file.name, { ...file });
}

async function upload(ev: Event) {
	const input = ev.target as HTMLInputElement;
	for (const file of input?.files || []) {
		addToast({
			data: {
				title: $localize("files-upload-started", {
					file: file.name,
				}),
				message: $localize("files-upload-started.message"),
				level: "info",
			},
		});
		const form = new FormData();
		form.append("path", `${data.path}/${file.name}`);

		if (file.type === "application/json") {
			form.append("file", await escapeJSONFile(file));
		} else {
			form.append("file", file);
		}

		// can't use openapi-fetch here because it doesn't handle
		// file uploads / multipart form data very well
		const response = await fetch("/api/files", {
			method: "POST",
			body: form,
		});

		if (!response.ok) {
			try {
				const error: ErrorResponse = await response.json();
				reportError(
					error,
					$localize("files-upload-failed", { file: file.name }),
				);
			} catch (e) {
				const err = e as Error;

				reportError(
					{ error: "json_response", message: err.message },
					$localize("files-upload-failed", { file: file.name }),
				);
			}
			continue;
		}

		addToast({
			data: {
				title: $localize("files-upload-succeeded", {
					file: file.name,
				}),
				message: $localize("files-upload-succeeded.message"),
				level: "info",
			},
		});
	}

	invalidate((url) => url.pathname === "/api/files");
}

async function download({ detail: file }: CustomEvent<File>) {
	const link = document.createElement("a");
	link.href = `/blob${file.path}`;
	link.download = "";
	link.click();
}

async function onDelete({ detail: file }: CustomEvent<File>) {
	if (!confirm($localize("files-delete-confirm", { file: file.name }))) {
		return;
	}

	return deleteFile(file);
}

async function deleteFile(file: File) {
	const { error, response } = await client.DELETE("/api/files/{id}", {
		params: { path: { id: file.id } },
	});
	if (error || !response.ok) {
		handleResponseError(error || response);
		return;
	}

	selected.update((selected) => {
		selected.delete(file);
		return selected;
	});

	if (!files) {
		return;
	}

	files = {
		...files,
		items: files.items.filter((f) => f.id !== file.id),
		count: files.count - 1,
	};
}

async function deleteSelected() {
	if (!confirm($localize("files-delete-confirm", { file: "selected" }))) {
		return;
	}

	const deletes = [...get(selected)].map((f) => deleteFile(f));
	await Promise.allSettled(deletes);
}

async function selectAll(ev: { currentTarget: HTMLInputElement }) {
	if (ev.currentTarget.checked) {
		selected.set(new Set(files?.items || []));
	} else {
		selected.set(new Set());
	}
}

async function handleResponseError(error: ErrorResponse | Response) {
	const data =
		error instanceof Response
			? { error: error.statusText, message: await error.text() }
			: error;
	reportError(data);
}

function togglePreview(preview?: File) {
	pushState("", { preview });
}

function createFolder() {
	const ff: FileList = files ?? {
		items: [],
		count: 0,
		total: 0,
		next: null,
	};

	const folder: File = {
		id: NEW_FOLDER_ID,
		type: "folder",
		contentType: "application/x-folder",
		path: "/new",
		name: "New Folder",
		size: 0,
	};

	files = {
		...ff,
		items: [folder, ...ff.items],
		count: ff.count + 1,
	};
}

async function rename({ detail: file }: CustomEvent<File>) {
	if (!files) return;

	const idx = files.items.findIndex(({ path }) => file.path === path);

	const path = renamePath(file.path, file.name);
	let updated: File | null = null;
	if (file.id !== NEW_FOLDER_ID) {
		const { data, error, response } = await client.PATCH("/api/files/{id}", {
			params: { path: { id: file.id } },
			body: { path },
		});

		if (error || !response.ok) {
			handleResponseError(error || response);
			return;
		}

		updated = data;
	} else {
		updated = {
			...file,
			path,
		};
	}

	const { items, ...ff } = files;
	items[idx] = updated;
	files = {
		...ff,
		items,
	};
}
</script>

<div class="action-bar">
    <div class="action-bar-left">
        <Localized id="files-title" let:text>
            <PageTitle title={text} />
            <h1 class="sr-only">{text}</h1>
        </Localized>

        <Localized id="files-title" let:text>
            <a href="/files" title={text}>
                <i class="fa-solid fa-house text-primary-900"></i>
            </a>
        </Localized>

        <Localized id="files-actions" let:attrs>
            <input
                class="checkbox"
                type="checkbox"
                on:change={selectAll}
                title={attrs["select-all"]}
                aria-label={attrs["select-all"]}
            />

            {#if $selected.size > 0}
                <button class="button filled" on:click={deleteSelected}>
                    <i class="fa-solid fa-trash"></i>
                    {attrs.delete}
                </button>
            {/if}
        </Localized>
    </div>

    <div class="action-bar-right">
        <Localized id="files-switch-view-mode" let:attrs>
            <button on:click={switchMode} title={attrs[viewMode]}>
                <i
                    class="text-primary-900 fa-solid"
                    class:fa-list-ul={viewMode === "grid"}
                    class:fa-border-all={viewMode === "list"}
                ></i>
            </button>
        </Localized>
    </div>
</div>

{#await data.response}
    <Loading />
{:then { data: files, error, response }}
    {#if error || !response.ok}
        {handleResponseError(error || response)}
    {:else}
        {setFiles(files)}
    {/if}
{/await}

{#if files}
    {#if files.count === 0}
        <Localized id="files-empty" let:text>
            <NoFiles {text} />
        </Localized>
    {:else}
        <svelte:component
            this={viewComponent}
            {files}
            selected={$selected}
            on:rename={rename}
            on:selected={select}
            on:deselected={deselect}
            on:download={download}
            on:delete={onDelete}
            on:preview={({ detail: file }) => togglePreview(file)}
        />
    {/if}
{/if}

{#if $page.state.preview}
    <FilePreview file={$page.state.preview} on:close={() => togglePreview()} />
{/if}

<Localized id="files-fab" let:text let:attrs>
    <Fab title={text} icon={faPlus} color="primary" multiple>
        <Fab
            title={attrs.folder}
            icon={faFolder}
            color="secondary"
            on:click={createFolder}
        />
        <Fab
            title={attrs.file}
            icon={faFile}
            color="secondary"
            on:click={pickFile}
        >
            <input
                data-testid="upload-files"
                bind:this={uploadInput}
                type="file"
                multiple
                on:change={upload}
                hidden
            />
        </Fab>
    </Fab>
</Localized>

<style>
    .action-bar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--oxi-size-4);
    }

    .action-bar-left {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        align-items: center;
        gap: var(--oxi-size-4);
    }
</style>
