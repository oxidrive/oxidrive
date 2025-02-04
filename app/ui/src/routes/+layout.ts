import { type Resources, loadResources } from "$lib/languages";
import type { SchemaAccountInfo } from "$lib/openapi";
import { error } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const ssr = false;

interface LayoutData {
	currentUser: SchemaAccountInfo | null;
	translations: {
		resources: Resources;
		languages: readonly string[];
	};
}

export const load: LayoutLoad<LayoutData> = async ({ fetch }) => {
	const translations = {
		resources: await loadResources(),
		languages: navigator.languages,
	};

	const currentUser = await loadCurrentSession(fetch);

	return {
		currentUser,
		translations,
	};
};

async function loadCurrentSession(
	fetch: Fetch,
): Promise<SchemaAccountInfo | null> {
	const resp = await fetch("/auth/session");

	if (resp.ok) {
		return resp.json();
	}

	if (resp.status === 401) {
		return null;
	}

	const body = await resp.text();

	const err = errorMessage(body);
	error(resp.status, err);
}

function errorMessage(body: string): { message: string } {
	try {
		// TODO: localize based on ApiError.error value
		return JSON.parse(body);
	} catch (e) {
		return { message: body };
	}
}
