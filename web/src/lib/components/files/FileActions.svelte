<script lang="ts">
import type { File, FileType } from "$lib/api";
import { createDropdownMenu, melt } from "@melt-ui/svelte";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { createEventDispatcher } from "svelte";

type Actions = "rename" | "download" | "delete";

interface Action {
	icon: `fa-${string} fa-${string}`;
	action: Actions;
	disabled?: true;
	type: FileType | "all";
}

const actions: Action[] = [
	{ action: "rename", icon: "fa-solid fa-pencil", type: "all" },
	{ action: "download", icon: "fa-solid fa-download", type: "file" },
	{ action: "delete", icon: "fa-solid fa-trash", type: "file" },
];

const dispatch = createEventDispatcher<Record<Actions, File>>();

let clazz = "";
export { clazz as class };

export let file: File;

const {
	elements: { menu, item, trigger, arrow },
} = createDropdownMenu();
</script>

<Localized id="files-actions" let:text let:attrs>
    <button
        use:melt={$trigger}
        title={text}
        class={clazz}
        data-testid="file-actions"
    >
        <i class="fa-solid fa-ellipsis text-primary-500"></i>
    </button>
    <div use:melt={$menu} class="file-actions">
        {#each actions.filter(({ type }) => type === 'all' || type === file.type) as { icon, action, disabled }}
            <button
                use:melt={$item}
                class="action"
                {disabled}
                on:click={() => dispatch(action, file)}
            >
                <i class={icon}></i>
                {attrs[action]}
            </button>
        {/each}
        <div use:melt={$arrow} class="arrow" />
    </div>
</Localized>

<style lang="scss">
    .file-actions {
        --border: 2px;
        z-index: 10;
        padding: var(--oxi-size-1);
        background-color: var(--oxi-color-primary-50);
        border: var(--border) solid var(--oxi-color-primary-500);
        border-radius: var(--oxi-rounded-2xl);

        .action {
            display: flex;
            flex-direction: row;
            gap: var(--oxi-size-2);
            justify-content: start;
            align-items: center;
            padding: var(--oxi-size-3);
            border-radius: var(--oxi-rounded-xl);
            width: 100%;

            &:disabled {
                pointer-events: none;
                color: var(--oxi-color-primary-300);
            }

            &:hover {
                background-color: var(--oxi-color-primary-100);
            }
        }

        .arrow {
            border-top: var(--border) solid var(--oxi-color-primary-500);
            border-left: var(--border) solid var(--oxi-color-primary-500);
        }
    }
</style>
