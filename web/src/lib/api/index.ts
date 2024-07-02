import createClient from "openapi-fetch";
import type { components, paths } from "./schema";

export type Client = ReturnType<typeof createClient<paths>>;

export const client = createClient<paths>({
	baseUrl: globalThis.location?.origin,
});

export class ApiError extends Error {
	readonly error: string;

	constructor({ error, message }: ErrorResponse) {
		super(message);
		this.error = error;
	}
}

export type ErrorResponse = components["schemas"]["Error"];
export type FileList = components["schemas"]["FileList"];
export type File = components["schemas"]["File"];
export type FileType = File["type"];
