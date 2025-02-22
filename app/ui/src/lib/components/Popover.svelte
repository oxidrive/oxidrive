<script lang="ts">
import { createPopover, melt } from "@melt-ui/svelte";
import { getFluentContext } from "@nubolab-ffwd/svelte-fluent";
import type { Snippet } from "svelte";

const { localize } = getFluentContext();

interface Snippets {
	button: Snippet;
	children: Snippet;
}

interface TranslatedLabel {
	labelId: string;
}

interface Label {
	label: string;
}

type Props = Snippets & (TranslatedLabel | Label);

const { button, children, ...props }: Props = $props();

function hasLabel(props: TranslatedLabel | Label): props is Label {
	return "label" in props;
}

const label = hasLabel(props) ? props.label : localize(props.labelId);

const {
	elements: { content, trigger, arrow },
	states: { open },
} = createPopover();
</script>


<button
    type="button"
    use:melt={$trigger}
    title={label}
    aria-label={label}>
    {@render button()}
</button>

{#if $open}
    <div class="popover" use:melt={$content}>
        <div class="arrow" use:melt={$arrow}></div>
        <div>
            {@render children()}
        </div>
    </div>
{/if}

<style lang="scss">
  @use "$lib/styles/oxi";

    .popover {
        background-color: oxi.primary(50);
        padding: oxi.size(2xs);
        border: 1px solid oxi.primary(300);
        border-radius: oxi.rounded(2xl);
    }

 .arrow {
     border-top: 1px solid oxi.primary(300);
     border-left: 1px solid oxi.primary(300);
 }
</style>
