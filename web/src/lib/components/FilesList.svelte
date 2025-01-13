<script lang="ts">
import type { SchemaFileData, SchemaPageFileData } from "$lib/openapi";
import { userTags } from "$lib/tags";
import FileActions from "./FileActions.svelte";
import Tag from "./Tag.svelte";

interface Props {
	ondelete: (file: SchemaFileData) => void;
	onpreview: (file: SchemaFileData) => void;
	files: SchemaPageFileData;
}

const { ondelete, onpreview, files }: Props = $props();

const sizes = [
	"byte",
	"kilobyte",
	"megabyte",
	"gigabyte",
	"terabyte",
	"petabyte",
];

function formatSize(size: number): string {
	const i = Math.floor(Math.log(size) / Math.log(1024));

	const formatter = new Intl.NumberFormat(undefined, {
		style: "unit",
		unit: sizes[i],
	});

	const bytes = size / 1024 ** i;

	return formatter.format(bytes);
}
</script>

<div class="files">
    {#each files.items as file (file.id)}
        {@const tags = userTags(file)}
        <div class="file">
            <input class="checkbox" type="checkbox" />
            <div class="info">
                <span class="header">
                    <button onclick={() => onpreview(file)} class="name truncate">
                        {file.name}
                    </button>

                    <div class="file-details">
                        <p>{formatSize(file.size)}</p>
                    </div>
                </span>

                {#if tags.length > 0}
                    <div class="file-tags">
                        {#each tags as tag (tag)}
                            <Tag {tag} size="small" />
                        {/each}
                    </div>
                {/if}
            </div>
            <div class="actions">
                <FileActions {file} {ondelete} />
            </div>
        </div>
    {/each}
</div>

<style>
    .files {
        display: flex;
        flex-direction: column;
        justify-content: stretch;
        align-items: center;
    }

    .file {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        padding: var(--oxi-size-2xs) var(--oxi-size-xs);
        border-bottom: 1px solid var(--oxi-primary-300);
        gap: var(--oxi-size-sm);
    }

    .file > .info {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: var(--oxi-size-2xs);
        flex: 1 1 0;
        min-width: 0;
    }

    .file > .info > .header {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: flex-start;
        gap: var(--oxi-size-4xs);
        max-width: 90%;
    }

    .name {
        text-align: start;
        width: 100%;
    }

    .file-tags {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: flex-start;
        align-items: center;
        column-gap: var(--oxi-size-2xs);
        row-gap: var(--oxi-size-4xs);
        width: 100%;

        & > :global(.tag) {
            max-width: 40%;
        }
    }

    .actions {
        flex: 0 1 0;
    }

    .file-details {
        font-size: var(--oxi-text-2xs-font-size);
        line-height: var(--oxi-text-2xs-line-height);
    }
</style>
