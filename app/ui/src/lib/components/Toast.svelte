<script lang="ts" context="module">
export type ToastData = {
	title: string;
	message: string;
	level: "info" | "danger";
};

const {
	elements: { content, title, description, close },
	helpers,
	states: { toasts },
	actions: { portal },
} = createToaster<ToastData>();

export const addToast = helpers.addToast;

export type AddToast = typeof addToast;

export function notify(data: Omit<ToastData, "level">) {
	addToast({
		data: {
			...data,
			level: "info",
		},
	});
}

export function reportError(
	error: { error: string; message: string },
	title: string = error.error,
) {
	addToast({
		data: {
			title,
			message: error.message,
			level: "danger",
		},
	});
}
</script>

<script lang="ts">
 import { createToaster, melt } from "@melt-ui/svelte";
 import { getFluentContext } from "@nubolab-ffwd/svelte-fluent";
 import { flip } from "svelte/animate";
 import { fly } from "svelte/transition";

 const { localize } = getFluentContext();

 const label = localize("toasts-close");
</script>

<div class="toasts" use:portal>
    {#each $toasts as { id, data } (id)}
        <div
            data-toast-level={data.level}
            class="toast {data.level}"
            use:melt={$content(id)}
            animate:flip={{ duration: 500 }}
            in:fly={{ duration: 150, x: "100%" }}
            out:fly={{ duration: 150, x: "100%" }}
        >
            <span class="header">
                <h3 use:melt={$title(id)} class="text-lg text-bold">
                    {data.title}
                </h3>
                <button use:melt={$close(id)} title={label} aria-label={label}>
                    <i class="fa-solid fa-xmark"></i>
                </button>
            </span>
            <div use:melt={$description(id)}>
                {data.message}
            </div>
        </div>
    {/each}
</div>

<style lang="scss">
 @use "$lib/styles/oxi";

 .toasts {
     position: fixed;
     right: 0;
     top: 0;
     z-index: 60;
     margin: oxi.size(sm);
     display: flex;
     flex-direction: column;
     gap: oxi.size(2xs);
     max-width: 90vw;
 }

 .toast {
     padding: oxi.size(2xs);
     border-radius: oxi.rounded(2xl);
     word-break: break-word;
     hyphens: auto;

     &.info {
         background-color: oxi.primary(50);
         border: 2px solid oxi.primary(500);
     }

     &.danger {
         background-color: oxi.danger(50);
         border: 2px solid oxi.danger(500);
     }

     .header {
         display: flex;
         flex-flow: row nowrap;
         justify-content: space-between;
         align-items: center;
     }
 }
</style>
