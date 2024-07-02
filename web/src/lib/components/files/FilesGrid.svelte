<script lang="ts">
import type { File, FileList } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { createEventDispatcher } from "svelte";
import FileActions from "./FileActions.svelte";
import FileIcon from "./FileIcon.svelte";
import FileLink from "./FileLink.svelte";

type InputEventHandler = (
	ev: Event & { currentTarget: EventTarget & HTMLInputElement },
) => void;

const dispatch = createEventDispatcher<{
	rename: File;
	selected: File;
	deselected: File;
}>();

export let files: FileList;
export let selected: Set<File>;

let renaming: File | null;

function select(file: File): InputEventHandler {
	return (ev) => {
		if (ev.currentTarget.checked) {
			dispatch("selected", file);
		} else {
			dispatch("deselected", file);
		}
	};
}

function startRenaming({ detail: file }: CustomEvent<File>) {
	if (renaming) {
		renaming = null;
	} else {
		renaming = file;
	}
}

function rename() {
	if (!renaming) return;

	dispatch("rename", renaming);
	renaming = null;
}

function focus(input: HTMLInputElement) {
	setTimeout(() => input.focus(), 100);
}
</script>

<div class="files-grid">
    {#each files.items as file}
        <div class="file-box" title={file.name}>
            <div class="header">
                <Localized id="files-selected" let:text>
                    <input
                        class="checkbox"
                        type="checkbox"
                        aria-label={text}
                        on:change={select(file)}
                        checked={selected.has(file)}
                    />
                </Localized>
            </div>

            <FileLink {file} on:preview>
                <FileIcon {file} height="80px" width="80px" />
            </FileLink>

            <div class="footer">
                {#if renaming === file}
                    <form on:submit|preventDefault={rename}>
                        <input class="input thin" type="text" bind:value={renaming.name} use:focus>
                    </form>
                {:else}
                    <FileLink {file} class="text-primary-500 truncate" on:preview>
                        {file.name}
                    </FileLink>
                {/if}
                <FileActions {file} on:rename={startRenaming} on:download on:delete />
            </div>
        </div>
    {/each}
</div>

<style lang="scss">
    .files-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
        padding: var(--oxi-size-4);
    }

    .file-box {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        aspect-ratio: 1 / 1;
        padding: var(--oxi-size-2);

        .header {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
        }

        .footer {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
            gap: var(--oxi-size-4);
            padding: var(--oxi-size-2);
        }
    }
</style>
