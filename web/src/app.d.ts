// See https://kit.svelte.dev/docs/types#app

import type { File } from "$lib/api";

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			preview: File | undefined;
		}
		// interface Platform {}
	}
}
