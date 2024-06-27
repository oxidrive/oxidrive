<script lang="ts">
import type { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
import { createDropdownMenu, melt } from "@melt-ui/svelte";
import { fly } from "svelte/transition";

export let color: "primary" | "secondary";
export let icon: IconDefinition;
export let title: string;
export let multiple = false;

const {
	elements: { trigger, menu, item },
	states: { open },
} = createDropdownMenu({ portal: null });
</script>

{#if multiple}
    <div class="fab-multi">
        <button use:melt={$trigger} class="fab {color}" {title}>
            <FontAwesomeIcon {icon} />
        </button>

        {#if $open}
            <div
                class="fab-children"
                use:melt={$menu}
                transition:fly={{ duration: 150, y: 10 }}
            >
                <slot />
            </div>
        {/if}
    </div>
{:else}
    <button class="fab {color}" {title} on:click use:melt={$item}>
        <FontAwesomeIcon {icon} />

        <slot />
    </button>
{/if}
