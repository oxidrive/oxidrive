<script lang="ts">
import { browser } from "$app/environment";
import type { SchemaFileData } from "$lib/openapi";
import { Localized } from "@nubolab-ffwd/svelte-fluent";
import { onMount } from "svelte";

interface Props {
	file: SchemaFileData;
	onclose: (file: SchemaFileData) => void;
}

let { file, onclose }: Props = $props();

const path = `/files/${file.name}`;

function close(ev: MouseEvent | KeyboardEvent) {
	ev.preventDefault();

	if (ev instanceof KeyboardEvent && ev.key !== "Escape") {
		return;
	}

	onclose(file);
}

async function isPreviewable(): Promise<boolean> {
	if (!browser) return false;

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

<svelte:document onkeydown={close} />

<div role="dialog" class="preview">
    <nav class="header">
        <span class="section spacer"></span>
        <h2 class="section name truncate">{file.name}</h2>
        <div class="section actions">
            <Localized id="files-preview-close">
                {#snippet children({ text })}
                    <button onclick={close} title={text} aria-label={text}>
                        <i class="fa-solid fa-xmark text-xl"></i>
                    </button>
                {/snippet}
            </Localized>
        </div>
    </nav>

    {#await isPreviewable() then showPreview}
        {#if showPreview}
            <Localized id="files-preqview">
                {#snippet children({ text })}
                    <iframe src={path} title={text} class="viewer"></iframe>
                {/snippet}
            </Localized>
        {:else}
            <div class="download">
                <Localized
                    id="files-preview-cannot-be-previewed"
                    args={{ file: file.name }}
                >
                    {#snippet children({ text, attrs })}
                        <p>{text}</p>
                        <a href={path} class="button filled">{attrs.download}</a>
                    {/snippet}
                </Localized>
            </div>
        {/if}
    {/await}
</div>

<style lang="scss">
 @use "$lib/styles/oxi";

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
     background-color: rgb(0 0 0 / 95%);
     gap: oxi.size(sm);

     .header {
         display: flex;
         flex-direction: row;
         justify-content: space-between;
         align-items: center;
         width: 100%;
         padding: oxi.size(sm);
         color: oxi.primary(50);

         .section {
             width: 32%;
         }

         .spacer {
             @media screen and (width <= 768px) {
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
             gap: oxi.size(2xl);
         }
     }

     .viewer {
         height: 100%;
         width: 100%;
     }

     .download {
         color: oxi.primary(50);
         width: 100%;
         height: 100%;
         padding: 0 oxi.size(xl);
         word-break: break-word;
         display: flex;
         flex-direction: column;
         justify-content: center;
         align-items: center;
         gap: oxi.size(sm);
     }
 }
</style>
