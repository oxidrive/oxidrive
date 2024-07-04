import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ parent, url }) => {
	const { session } = await parent();
	if (!session) throw redirect(303, `/login?redirect=${url.pathname}`);

	return {};
};
