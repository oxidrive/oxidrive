import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ parent }) => {
	const { currentUser } = await parent();
	if (!currentUser) throw redirect(303, "/ui/login");
};
