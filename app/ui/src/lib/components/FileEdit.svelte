<script lang="ts">
import { invalidate, invalidateAll } from "$app/navigation";
import { client } from "$lib/api";
import type { SchemaFileData, SchemaTag } from "$lib/openapi";
import { concat, equals, not } from "$lib/tags";
import { createDialog, melt } from "@melt-ui/svelte";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import type { Snippet } from "svelte";
import TagsInput from "./TagsInput.svelte";

interface Props {
	file: SchemaFileData;
	class?: string;
	children: Snippet;
}

let { file, class: clazz = "", children }: Props = $props();

const {
	elements: { trigger, overlay, content, title, close, portalled },
	states: { open },
} = createDialog({
	forceVisible: true,
});

let tags = $state([...file.tags]);

const add = (tag: SchemaTag) => {
	tags.push(tag);
};

const remove = (tag: SchemaTag) => {
	tags = tags.filter(not(equals(tag)));
};

const submit = async (ev: Event) => {
	ev.preventDefault();

	const { error } = await client.PUT("/api/v1/files/{file_id}/tags", {
		params: { path: { file_id: file.id } },
		body: { tags: tags.map(concat) },
	});

	if (error) {
		reportError(error);
		return;
	}

	await invalidate("/api/v1/files");

	open.set(false);
};
</script>

<button
    use:melt={$trigger}
    class={clazz}
>
{@render children()}
</button>

{#if $open}
    <div use:melt={$portalled}>
        <div use:melt={$overlay} class="overlay"></div>
        <div use:melt={$content} class="dialog">
            <Localized id="files-edit" args={{ file: file.name }}>
                {#snippet children({ text, attrs })}
                    <div class="header">
                        <h2 class="text-lg text-bold" use:melt={$title}>{text}</h2>
                        <button use:melt={$close} title={attrs.close} aria-label={attrs.close}>
                            <i class="fa-solid fa-xmark"></i>
                        </button>
                    </div>

                    <form class="form" onsubmit={submit}>
                        <div class="fieldset inputs">
                            <input class="input" type="text" name="name" bind:value={file.name} disabled>
                            <TagsInput tags={file.tags}
                                       onadd={add}
                                       onremove={remove} />
                        </div>

                        <div class="fieldset inline buttons">
                            <button class="button filled" type="submit">{attrs.submit}</button>
                            <button class="button outline" type="button" use:melt={$close}>{attrs.cancel}</button>
                        </div>
                    </form>
                {/snippet}
            </Localized>
        </div>
  </div>
{/if}

<style>
 .overlay {
     position: fixed;
     z-index: 50;
     inset: 0px;
     background-color: rgb(0, 0, 0, 0.75);
 }

 .dialog {
     position: fixed;
     left: 0;
     top: 40%;
     margin: var(--oxi-size-sm);
     padding: var(--oxi-size-sm);
     z-index: 50;
     max-height: 85vh;
     width: 90vw;
     max-width: 450px;
     border-radius: var(--oxi-rounded-xl);
     background-color: var(--oxi-white);
 }

.header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--oxi-size-sm);
}
</style>
