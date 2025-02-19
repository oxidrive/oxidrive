<script lang="ts">
import icon from "$lib/assets/collection.svg";
import Fab from "$lib/components/Fab.svelte";
import { faPlus, faTags } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

const { data }: Props = $props();
</script>

{#if data.collections.items.length > 0}
    <div class="collections">
        {#each data.collections.items as collection (collection.id)}
            <a class="collection" href={`/ui/collections/${collection.id}`}>
                <FontAwesomeIcon icon={faTags} class="collection-icon" />
                <div class="footer">
                    <p>{collection.name}</p>
                    <p>
                        <Localized
                            id="collections-files-count"
                            args={{ count: collection.files.length }}
                        />
                    </p>
                </div>
            </a>
        {/each}
    </div>
{:else}
    <div class="empty">
        <p><Localized id="collections-empty" /></p>
    </div>
{/if}

<Fab color="primary" icon={faPlus} label="upload" to="/ui/collections/new" />

<style>
    .collections {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
        padding: var(--oxi-size-sm);
        gap: var(--oxi-size-m);
    }

    .collection {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        gap: var(--oxi-size-sm);

        & > .footer {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
        }
    }

 :global(svg.collection-icon) {
     height: 140px;
 }

    .empty {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        padding: var(--oxi-size-sm);
    }
</style>
