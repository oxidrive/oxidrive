import { client } from "$lib/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
	const path = `/${params.path}`;
	return {
		path,
		response: client.GET("/api/files", {
			fetch,
			params: { query: { prefix: path } },
		}),
	};
};
