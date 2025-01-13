// See https://svelte.dev/docs/kit/types#app.d.ts

import type { SchemaFileData } from "$lib/openapi";

// for information about these interfaces
declare global {
	type Fetch = typeof globalThis.fetch;

	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			preview?: SchemaFileData;
			search?: string;
		}
		// interface Platform {}
	}
}
