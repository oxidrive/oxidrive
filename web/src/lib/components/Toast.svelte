<script lang="ts" context="module">
import type { ApiError, ErrorResponse } from "$lib/api";

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

export function notify(data: Omit<ToastData, "level">) {
	addToast({
		data: {
			...data,
			level: "info",
		},
	});
}

export function reportError(
	error: ErrorResponse | ApiError,
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
    import { Localized } from "@nubolab-ffwd/svelte-fluent";
    import { flip } from "svelte/animate";
    import { fly } from "svelte/transition";
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
                <Localized id="toasts-close" let:text>
                    <button use:melt={$close(id)} title={text}>
                        <i class="fa-solid fa-xmark"></i>
                    </button>
                </Localized>
            </span>
            <div use:melt={$description(id)}>
                {data.message}
            </div>
        </div>
    {/each}
</div>

<style lang="scss">
    .toasts {
        position: fixed;
        right: 0;
        top: 0;
        z-index: 60;
        margin: var(--oxi-size-4);
        display: flex;
        flex-direction: column;
        gap: var(--oxi-size-2);
        max-width: 90vw;
    }

    .toast {
        padding: var(--oxi-size-2);
        border-radius: var(--oxi-rounded-2xl);
        word-break: break-word;
        hyphens: auto;

        &.info {
            background-color: var(--oxi-color-primary-50);
            border: 2px solid var(--oxi-color-primary-500);
        }

        &.danger {
            background-color: var(--oxi-color-danger-50);
            border: 2px solid var(--oxi-color-danger-500);
        }

        .header {
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            justify-content: space-between;
            align-items: center;
        }
    }
</style>
