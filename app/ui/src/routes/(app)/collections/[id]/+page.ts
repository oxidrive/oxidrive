import { client } from "$lib/api";
import type { SchemaCollectionData, SchemaPageFileData } from "$lib/openapi";
import { error as throwError } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export interface PageData {
	collection: SchemaCollectionData;
	lazy: {
		files: Promise<SchemaPageFileData>;
	};
}

export const load: PageLoad<PageData> = async ({ fetch, params }) => {
	const collection = await loadCollection(params.id, fetch);
	const files = loadFiles(collection.files, fetch);

	return { collection, lazy: { files } };
};

async function loadCollection(
	id: string,
	fetch: Fetch,
): Promise<SchemaCollectionData> {
	const { data, error, response } = await client.GET(
		"/api/v1/collections/{collection_id}",
		{
			params: {
				path: {
					collection_id: id,
				},
			},
			fetch,
		},
	);

	if (error) {
		throwError(response.status, error);
	}

	return data;
}

async function loadFiles(
	ids: string[],
	fetch: Fetch,
): Promise<SchemaPageFileData> {
	if (ids.length === 0) {
		return { items: [], next: null, previous: null };
	}

	const { data, error, response } = await client.GET("/api/v1/files", {
		params: {
			query: {
				id: ids,
			},
		},
		fetch,
	});

	if (error) {
		throwError(response.status, error);
	}

	return data;
}
