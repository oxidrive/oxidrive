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
                    <button onclick={() => onpreview($state.snapshot(file))} class="name truncate">
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

<style lang="scss">
 @use "$lib/styles/oxi";

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
        padding: oxi.size(2xs) oxi.size(xs);
        border-bottom: 1px solid oxi.primary(300);
        gap: oxi.size(sm);
    }

    .file > .info {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: oxi.size(2xs);
        flex: 1 1 0;
        min-width: 0;
    }

    .file > .info > .header {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: flex-start;
        gap: oxi.size(4xs);
        max-width: 90%;
    }

    .name {
        text-align: start;
        width: 100%;
    }

    .file-tags {
        display: flex;
        flex-flow: row wrap;
        justify-content: flex-start;
        align-items: center;
        gap: oxi.size(4xs) oxi.size(2xs);
        width: 100%;

        & > :global(.tag) {
            max-width: 40%;
        }
    }

    .actions {
        flex: 0 1 0;
    }

    .file-details {
        @include oxi.text(2xs);
    }
</style>
