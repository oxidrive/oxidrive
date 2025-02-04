<script lang="ts">
import icon from "$lib/assets/collection.svg";
import Fab from "$lib/components/Fab.svelte";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import type { PageData } from "./$types";

interface Props {
	data: PageData;
}

const { data }: Props = $props();
</script>

<div class="collections">
    {#each data.collections.items as collection (collection.id)}
        <a class="collection" href={`/ui/collections/${collection.id}`}>
            <img class="icon" src={icon} alt={collection.name} />
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

<Fab
    color="primary"
    icon={faPlus}
    label="upload"
    to="/ui/collections/new"
></Fab>

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

        & > .icon {
            max-width: 140px;
        }

        & > .footer {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
        }
    }
</style>
