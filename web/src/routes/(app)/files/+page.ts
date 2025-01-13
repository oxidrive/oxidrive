import { client } from "$lib/api";
import type { SchemaPageFileData } from "$lib/openapi";
import { error as throwError } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export interface PageData {
	files: SchemaPageFileData;
	search: string | null;
	queryError?: string;
}

export const load: PageLoad<PageData> = async ({ fetch, url }) => {
	const search = url.searchParams.get("search");

	const { data, error, response } = await client.GET("/api/v1/files", {
		params: {
			query: {
				search,
			},
		},
		fetch,
	});

	if (error) {
		switch (error.error) {
			case "INVALID_QUERY":
				return {
					files: { items: [], next: null, previous: null },
					search,
					queryError: error.message,
				};
			default:
				throwError(response.status, error);
		}
	}

	return { files: data, search };
};
