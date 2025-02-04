import { browser } from "$app/environment";
import createClient from "openapi-fetch";
import type { paths } from "./openapi";

export const client = createClient<paths>({
	baseUrl: browser ? window.origin : undefined,
});
