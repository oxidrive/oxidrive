<script lang="ts">
import type { SchemaFileData } from "$lib/openapi";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import FileEdit from "./FileEdit.svelte";
import Popover from "./Popover.svelte";

interface Props {
	ondelete: (file: SchemaFileData) => void;
	file: SchemaFileData;
}

const { ondelete, file }: Props = $props();
</script>

<Popover labelId="files-actions">
    {#snippet button()}
        <i class="fa-solid fa-ellipsis text-2xl"></i>
    {/snippet}

    <div class="actions">
        <FileEdit class="action" {file}>
            <i class="fa-solid fa-pencil"></i>
            <Localized id="files-actions.edit" />
        </FileEdit>
        <a class="action" href={`/files/${file.name}?force=true`} target="_blank">
            <i class="fa-solid fa-download"></i>
            <Localized id="files-actions.download" />
        </a>
        <button class="action" onclick={() => ondelete(file)}>
            <i class="fa-solid fa-trash"></i>
            <Localized id="files-actions.delete" />
        </button>
    </div>
</Popover>

<style>
 .actions {
     display: flex;
     flex-direction: column;
     justify-content: stretch;
     align-items: center;
     gap: var(--oxi-size-2xs);
 }

 .actions > :global(.action) {
     display: flex;
     flex-direction: row;
     justify-content: flex-start;
     align-items: center;
     width: 100%;
     gap: var(--oxi-size-sm);
     color: var(--oxi-primary-500);

     &:disabled {
         color: var(--oxi-primary-300);
     }
 }
</style>
