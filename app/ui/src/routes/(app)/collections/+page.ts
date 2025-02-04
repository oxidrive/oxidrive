import { client } from "$lib/api";
import type { SchemaPageCollectionData } from "$lib/openapi";
import { error as throwError } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export interface PageData {
	collections: SchemaPageCollectionData;
}

export const load: PageLoad<PageData> = async ({ fetch }) => {
	const { data, error, response } = await client.GET("/api/v1/collections", {
		fetch,
	});
	if (error) {
		throwError(response.status, error);
	}

	return { collections: data };
};
