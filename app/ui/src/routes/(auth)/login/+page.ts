import type { PageLoad } from "./$types";

export interface PageData {
	error: string | null;
	redirect_to: string;
}

export const load: PageLoad<PageData> = async ({ url }) => {
	const error = url.searchParams.get("error");
	const redirect_to = url.searchParams.get("redirect_to") || "/ui";

	return { error, redirect_to };
};
