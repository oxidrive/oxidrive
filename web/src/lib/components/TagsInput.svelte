<script lang="ts">
import type { SchemaTag } from "$lib/openapi";
import { excludeSystemTags, split } from "$lib/tags";
import { createTagsInput, melt } from "@melt-ui/svelte";
import { Localized } from "@nubolab-ffwd/svelte-fluent";

interface Props {
	tags: SchemaTag[];
	onadd?: (tag: SchemaTag) => void;
	onremove?: (tag: SchemaTag) => void;
}

const {
	tags: tagList,
	onadd = () => {},
	onremove = () => {},
}: Props = $props();

const defaultTags = tagList
	.filter(excludeSystemTags)
	.map(({ key, value }) => (value ? `${key}:${value}` : key));

const {
	elements: { root, input, tag, deleteTrigger },
	states: { tags },
} = createTagsInput({
	defaultTags,
	unique: true,
	editable: false,
	addOnPaste: true,
	add: (tag) => {
		const t = tag.replaceAll(" ", "");
		onadd(split(t));
		return t;
	},
	remove: (tag) => {
		onremove(split(tag.value));
		return true;
	},
});
</script>

<Localized id="tags-input">
    {#snippet children({ text, attrs })}
        <div use:melt={$root} class="tags">
            <label for="tags" class="sr-only">{text}</label>
            <input use:melt={$input} class="input" type="text" name="tags" placeholder={text}/>

            {#each $tags as t (t)}
                {@const { key, value } = split(t.value)}
                <div use:melt={$tag(t)} class="tag">
                    <span class="key"
                    >{key}{#if value}:{/if}</span
                                           >
                    {#if value}<span class="value">{value}</span>{/if}

                    <button class="delete"
                            type="button"
                            use:melt={$deleteTrigger(t)}
                            title={attrs.delete}
                            aria-label={attrs.delete}>
                        <i class="fa-solid fa-xmark"></i>
                    </button>
                </div>
            {/each}
        </div>
    {/snippet}
</Localized>

<style>
    .tags {
        width: 100%;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: var(--oxi-size-2xs);
    }

    .tags > :global(.tag) {
        max-width: 100%;
    }

    .delete {
        margin-left: var(--oxi-size-2xs);
    }
</style>
