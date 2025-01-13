<script lang="ts">
import { createDropdownMenu, melt } from "@melt-ui/svelte";
import { getFluentContext } from "@nubolab-ffwd/svelte-fluent";
import type { Snippet } from "svelte";

const { localize } = getFluentContext();

const {
	elements: { menu, item, trigger, arrow },
} = createDropdownMenu();

interface Snippets {
	button: Snippet;
	children: Snippet<[typeof $item]>;
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
</script>


<button
    type="button"
    use:melt={$trigger}
    title={label}
    aria-label={label}>
    {@render button()}
</button>

<div use:melt={$menu} class="menu">
    {@render children($item)}

    <div use:melt={$arrow} ></div>
</div>

<style>
    .menu {
        background-color: var(--oxi-primary-50);
        padding: var(--oxi-size-2xs);
        border: 1px solid var(--oxi-primary-300);
        border-radius: var(--oxi-rounded-2xl);
    }
</style>
