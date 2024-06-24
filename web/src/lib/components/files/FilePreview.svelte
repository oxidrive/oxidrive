<script lang="ts">
import type { File } from "$lib/api";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { createEventDispatcher, onMount } from "svelte";

const dispatch = createEventDispatcher();

export let file: File;

const path = `/blob${file.path}`;

function close(ev: MouseEvent | KeyboardEvent) {
	if (ev instanceof KeyboardEvent && ev.key !== "Escape") {
		return;
	}

	dispatch("close", file);
}

async function isPreviewable(): Promise<boolean> {
	const response = await fetch(path, {
		method: "HEAD",
		cache: "force-cache",
	});
	return response.headers.get("content-disposition") === "inline";
}

onMount(() => {
	const previous = document.body.style.overflow;
	document.body.style.overflow = "hidden";

	return () => {
		document.body.style.overflow = previous;
	};
});
</script>

<svelte:document on:keydown|preventDefault={close} />

<div role="dialog" class="preview">
    <nav class="header">
        <span class="section spacer"></span>
        <h2 class="section name truncate">{file.name}</h2>
        <div class="section actions">
            <Localized id="files-preview-close" let:text>
                <button on:click={close} title={text}>
                    <i class="fa-solid fa-xmark text-xl"></i>
                </button>
            </Localized>
        </div>
    </nav>

    {#await isPreviewable() then showPreview}
        {#if showPreview}
            <Localized id="files-preview" let:text>
                <iframe src={path} title={text} class="viewer" />
            </Localized>
        {:else}
            <div class="download">
                <Localized
                    id="files-preview-cannot-be-previewed"
                    let:text
                    let:attrs
                    args={{ file: file.name }}
                >
                    <p>{text}</p>
                    <a href={path} class="button filled">{attrs.download}</a>
                </Localized>
            </div>
        {/if}
    {/await}
</div>

<style lang="scss">
    .preview {
        position: fixed;
        top: 0;
        height: 100%;
        width: 100%;
        z-index: 20;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: start;
        background-color: rgb(0, 0, 0, 0.95);
        gap: var(--oxi-size-4);

        .header {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
            padding: var(--oxi-size-4);
            color: var(--oxi-color-primary-50);

            .section {
                width: 32%;
            }

            .spacer {
                @media screen and (max-width: 768px) {
                    display: none;
                }
            }

            .name {
                text-align: center;
            }

            .actions {
                display: flex;
                flex-direction: row;
                justify-content: end;
                align-items: center;
                gap: var(--oxi-size-2);
            }
        }

        .viewer {
            height: 100%;
            width: 100%;
        }

        .download {
            color: var(--oxi-color-primary-50);
            width: 100%;
            height: 100%;
            padding: 0 var(--oxi-size-8);
            word-break: break-word;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            gap: var(--oxi-size-4);
        }
    }
</style>
