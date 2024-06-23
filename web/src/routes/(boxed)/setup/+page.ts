import { ApiError, client } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
	const { data, error } = await client.GET("/api/instance", { fetch });
	if (error) throw new ApiError(error);

	if (data.status.setupCompleted) throw redirect(303, "/");

	return data;
};
