import { client } from "$lib/api";
import type { SchemaAccountInfo } from "$lib/openapi";
import { error as throwError } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

interface AccountData {
	account: SchemaAccountInfo;
}

export const load: PageLoad<AccountData> = async ({ fetch }) => {
	const { data, error, response } = await client.GET("/api/v1/accounts/me", {
		fetch,
	});

	if (error) {
		throwError(response.status, error);
	}

	return { account: data };
};
