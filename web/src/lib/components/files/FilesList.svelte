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

let renaming: File | null = null;

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

<div class="files-list">
    {#each files.items as file}
        <div class="row" title={file.name}>
            <div class="file">
                <span class="header">
                    <Localized id="files-selected" let:text>
                        <input
                            class="checkbox"
                            type="checkbox"
                            aria-label={text}
                            on:change={select(file)}
                            checked={selected.has(file)}
                        />
                    </Localized>

                    <FileLink {file} on:preview>
                        <FileIcon {file} height="40px" width="40px" />
                    </FileLink>

                    {#if renaming === file}
                        <form on:submit|preventDefault={rename}>
                            <input class="input thin" type="text" bind:value={renaming.name} use:focus>
                        </form>
                    {:else}
                        <FileLink {file} class="text-primary-500 truncate" on:preview>
                            {file.name}
                        </FileLink>
                    {/if}
                </span>

                <FileActions
                    class="actions"
                    {file}
                    on:rename={startRenaming}
                    on:download
                    on:delete
                />
            </div>
            <hr class="sep" />
        </div>
    {/each}
</div>

<style lang="scss">
    .files-list {
        display: flex;
        flex-direction: column;
        padding: var(--oxi-size-4);
    }

    .row {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin: var(--oxi-size-2);
        gap: var(--oxi-size-2);

        .file {
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            justify-content: space-between;
            align-items: center;
            width: 100%;

            .header {
                display: flex;
                flex-direction: row;
                flex-wrap: nowrap;
                justify-content: start;
                align-items: center;
                gap: var(--oxi-size-4);
                max-width: 90%;
            }

            :global(.actions) {
                flex-grow: 0;
                flex-shrink: 0;
            }
        }

        .sep {
            width: 100%;
            height: 1px;
            background-color: var(--oxi-color-primary-300);

            &:last-child {
                display: none;
            }
        }
    }
</style>
