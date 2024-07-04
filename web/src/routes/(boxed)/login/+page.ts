import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent, url }) => {
	const { session } = await parent();
	const to = url.searchParams.get("redirect") ?? "/";

	if (session) {
		throw redirect(303, to);
	}

	return { to };
};
