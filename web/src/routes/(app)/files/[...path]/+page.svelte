<script lang="ts">
import { goto, invalidate } from "$app/navigation";
import type { ErrorResponse, File } from "$lib/api";
import ActionBar from "$lib/components/ActionBar.svelte";
import Loading from "$lib/components/Loading.svelte";
import PageTitle from "$lib/components/PageTitle.svelte";
import { addToast, reportError } from "$lib/components/Toast.svelte";
import FilePreview from "$lib/components/files/FilePreview.svelte";
import FilesGrid from "$lib/components/files/FilesGrid.svelte";
import FilesList from "$lib/components/files/FilesList.svelte";
import NoFiles from "$lib/components/files/NoFiles.svelte";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
import { Localized, localize } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

let viewMode: "list" | "grid" = "grid";
let viewComponent = FilesGrid;

export let data: PageData;

let uploadInput: HTMLInputElement;

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
		form.append("file", file);

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

async function handleResponseError(error: ErrorResponse | Response) {
	const data =
		error instanceof Response
			? { error: error.statusText, message: await error.text() }
			: error;
	reportError(data);
}
</script>

<div class="action-bar">
    <Localized id="files-title" let:text>
        <PageTitle title={text} />
        <h1 class="sr-only">{text}</h1>
    </Localized>

    <Localized id="files-title" let:text>
        <a href="/files" title={text}>
            <i class="fa-solid fa-house text-primary-900"></i>
        </a>
    </Localized>

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

{#await data.response}
    <Loading />
{:then { data: files, error, response }}
    {#if error || !response.ok}
        {handleResponseError(error || response)}
    {:else if files.count === 0}
        <Localized id="files-empty" let:text>
            <NoFiles {text} />
        </Localized>
    {:else}
        <svelte:component this={viewComponent} {files} on:download={download} />
        {@const fileToPreview = files.items.find(
            (f) => data.preview && f.name === data.preview,
        )}
        {#if fileToPreview}
            <FilePreview
                file={fileToPreview}
                on:close={() => goto(window.location.pathname)}
            />
        {/if}
    {/if}
{/await}

<Localized id="files-upload-cta" let:text>
    <button class="fab primary" on:click={pickFile} title={text}>
        <FontAwesomeIcon icon={faPlus} />
        <input
            data-testid="upload-files"
            bind:this={uploadInput}
            type="file"
            multiple
            on:change={upload}
            hidden
        />
    </button>
</Localized>

<style>
    .action-bar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--oxi-size-4);
    }
</style>
