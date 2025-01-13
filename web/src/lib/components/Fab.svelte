<script lang="ts">
import type { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
import { createDropdownMenu, melt } from "@melt-ui/svelte";
import type { Snippet } from "svelte";
import type { MouseEventHandler } from "svelte/elements";
import { fly } from "svelte/transition";

interface Props {
	color: "primary" | "secondary";
	icon: IconDefinition;
	label: string;
	multiple?: boolean;
	onclick: MouseEventHandler<HTMLButtonElement>;
	children?: Snippet;
}

const {
	color,
	icon,
	label,
	multiple = false,
	onclick,
	children,
}: Props = $props();

const {
	elements: { trigger, menu, item },
	states: { open },
} = createDropdownMenu({ portal: null });
</script>

{#if multiple}
    <div class="fab-multi">
        <button use:melt={$trigger} class="fab {color}" title={label} aria-label={label}>
            <FontAwesomeIcon {icon} />
        </button>

        {#if $open}
            <div
                class="fab-children"
                use:melt={$menu}
                transition:fly={{ duration: 150, y: 10 }}
            >
                {@render children?.()}
            </div>
        {/if}
    </div>
{:else}
    <button class="fab {color}" title={label} aria-label={label} {onclick} use:melt={$item}>
        <FontAwesomeIcon {icon} />

        {@render children?.()}
    </button>
{/if}
