<script lang="ts">
import type { SchemaFileData, SchemaPageFileData } from "$lib/openapi";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import FileActions from "./FileActions.svelte";
import FileIcon from "./FileIcon.svelte";

interface Props {
	ondelete: (file: SchemaFileData) => void;
	onpreview: (file: SchemaFileData) => void;
	files: SchemaPageFileData;
}

const { ondelete, onpreview, files }: Props = $props();
</script>

<div class="files-grid">
    {#each files.items as file (file.id)}
        <div class="file-box" title={file.name}>
            <div class="header">
                <Localized id="files-selected">
                    {#snippet children({ text, attrs })}
                        <input
                            class="checkbox"
                            type="checkbox"
                            aria-label={text}
                        />
                    {/snippet}
                </Localized>
            </div>

            <button onclick={() => onpreview(file)}>
                <FileIcon {file} height="80px" width="80px" />
            </button>


            <div class="footer">
                <button onclick={() => onpreview($state.snapshot(file))}>
                    {file.name}
                </button>
                <FileActions {file} {ondelete} />
            </div>
        </div>
    {/each}
</div>

<style>
    .files-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
        padding: var(--oxi-size-sm);
    }

    .file-box {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        aspect-ratio: 1 / 1;
        padding: var(--oxi-size-2xs);

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
            gap: var(--oxi-size-sm);
            padding: var(--oxi-size-2xs);

        }
    }
</style>
