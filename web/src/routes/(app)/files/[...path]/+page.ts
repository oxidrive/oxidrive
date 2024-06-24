import { client } from "$lib/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params, url }) => {
	const path = `/${params.path}`;

	let preview = url.searchParams.get("preview");
	if (preview?.startsWith("/")) {
		preview = preview.replace("/", "");
	}

	return {
		path,
		preview,
		response: client.GET("/api/files", {
			fetch,
			params: { query: { prefix: path } },
		}),
	};
};
