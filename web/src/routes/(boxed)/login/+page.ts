import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent, url }) => {
	const { user } = await parent();
	const to = url.searchParams.get("redirect") ?? "/";

	if (user) {
		throw redirect(303, to);
	}

	return { to };
};
