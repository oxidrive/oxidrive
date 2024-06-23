import { ApiError, client } from "$lib/api";
import { loadResources } from "$lib/languages";
import type { LayoutLoad } from "./$types";

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
	const langs = {
		resources: await loadResources(),
		languages: navigator.languages,
	};

	const { data, error, response } = await client.GET("/api/session", { fetch });

	if (response.status === 401) {
		return { user: null, ...langs };
	}

	if (error) {
		throw new ApiError(error);
	}

	return { user: data, ...langs };
};
