<script lang="ts">
import type { FileList } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import FileIcon from "./FileIcon.svelte";
import FileLink from "./FileLink.svelte";

export let files: FileList;
</script>

<div class="files-grid">
    {#each files.items as file}
        <div class="file-box" title={file.name}>
            <div class="header">
                <Localized id="files-selected" let:text>
                    <input type="checkbox" aria-label={text} />
                </Localized>
            </div>

            <FileLink {file}>
                <FileIcon {file} height="80px" width="80px" />
            </FileLink>

            <div class="footer">
                <FileLink {file}>
                    <p class="text-primary-500 truncate">
                        {file.name}
                    </p>
                </FileLink>
                <i class="fa-solid fa-ellipsis text-primary-500"></i>
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
            justify-content: justify-between;
            align-items: center;
            width: 100%;
        }

        .footer {
            display: flex;
            flex-direction: row;
            justify-content: justify-between;
            align-items: center;
            width: 100%;
            gap: var(--oxi-size-4);
            padding: var(--oxi-size-2);
        }
    }
</style>
