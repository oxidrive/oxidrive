<script lang="ts">
import type { FileList } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import FileActions from "./FileActions.svelte";
import FileIcon from "./FileIcon.svelte";
import FileLink from "./FileLink.svelte";

export let files: FileList;
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
                        />
                    </Localized>

                    <FileLink {file}>
                        <FileIcon {file} height="40px" width="40px" />
                    </FileLink>

                    <FileLink {file} class="text-primary-500 truncate">
                        {file.name}
                    </FileLink>
                </span>

                <FileActions
                    class="actions"
                    {file}
                    on:rename
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
