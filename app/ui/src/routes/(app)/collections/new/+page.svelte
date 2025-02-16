<script lang="ts">
import { goto } from "$app/navigation";
import { client } from "$lib/api";
import Button from "$lib/components/Button.svelte";
import Filter from "$lib/components/Filter.svelte";
import { debounce } from "$lib/utils";
import { Localized, getFluentContext } from "@nubolab-ffwd/svelte-fluent";
import { z } from "zod";

const { localize } = getFluentContext();

const customErrorMap: z.ZodErrorMap = (issue, ctx) => {
	const field = issue.path.join(".");

	switch (issue.code) {
		case z.ZodIssueCode.too_small:
			return {
				message: localize("errors-validation.too-small", {
					field,
					min: issue.minimum as number,
				}),
			};
		default:
			return { message: ctx.defaultError };
	}
};

z.setErrorMap(customErrorMap);

const Data = z.object({
	name: z.string().trim().min(1),
	filter: z.string().trim().min(1),
});

let name = $state("");
let filter = $state("");

let matching = $state(0);

let errors: { name?: string; filter?: string } = $state({
	name: undefined,
	filter: undefined,
});

async function validate(): Promise<boolean> {
	const result = await Data.safeParseAsync({ name, filter });
	if (result.success) {
		errors = {};
		return true;
	}

	const formatted = result.error.format();
	errors.name = formatted.name?._errors.join(", ");
	errors.filter = formatted.filter?._errors.join(", ");
	return false;
}

let loading = $state(false);

async function submit(e: SubmitEvent) {
	e.preventDefault();

	if (!(await validate())) {
		return;
	}

	console.log("creating collection", { name, filter });
	loading = true;

	const { data, error } = await client.POST("/api/v1/collections", {
		body: { name, filter },
	});
	loading = false;

	if (error) {
		reportError(error);
		return;
	}

	await goto(`/ui/collections/${data.id}`);
}

const matchingFiles = debounce(async () => {
	const { data, error } = await client.GET("/api/v1/files", {
		params: { query: { search: filter } },
	});

	if (error) {
		switch (error.error) {
			case "INVALID_QUERY":
				errors.filter = error.message;
				break;
			default:
				reportError(error);
		}
		return;
	}

	// todo: handle pagination by adding a total count in the API
	matching = data?.items?.length || 0;
}, 500);

async function onFilterChanged() {
	Promise.all([validate(), matchingFiles()]);
}
</script>

<Localized id="new-collection-form">
{#snippet children({ text, attrs })}
<div class="create">
    <h1 class="title">{text}</h1>

    <form class="form" onsubmit={submit}>
        <div class="fieldset inputs">
            <div class="field">
                <label for="name" class="sr-only">{attrs.name}</label>
                <input
                    class="input"
                    class:error={!!errors.name}
                    type="text"
                    name="name"
                    placeholder={attrs.name}
                    bind:value={name}
                    oninput={validate}
                />
                {#if errors.name}<p class="error">{errors.name}</p>{/if}
            </div>

            <div class="field">
                <label for="filter" class="sr-only">{attrs.filter}</label>
                <input
                    class="input"
                    class:error={!!errors.filter}
                    type="text"
                    name="filter"
                    placeholder={attrs.filter}
                    bind:value={filter}
                    oninput={onFilterChanged}
                />
                {#if errors.filter}<p class="error">{errors.filter}</p>{/if}
                {#if filter}<p><Localized id="new-collection-matching-files" args={{ count: matching }} /></p>{/if}
            </div>
        </div>

        <div class="fieldset buttons">
            <Button variant="filled" type="submit" {loading}>{attrs.submit}</Button>
        </div>
    </form>
</div>
{/snippet}
</Localized>

<style>
    .create {
        padding: var(--oxi-size-m);
    }

    .title {
        margin-bottom: var(--oxi-size-xs);
        width: 100%;
        text-align: center;
    }
</style>
