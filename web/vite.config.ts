import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
	plugins: [sveltekit()],

	css: {
		preprocessorOptions: {
			scss: {
				api: "modern-compiler",
				quietDeps: true, // fontawesome is still using @import
			},
		},
	},

	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"],
	},

	server: {
		proxy: {
			"/api": "http://localhost:4000",
			"/auth": "http://localhost:4000",
			"/files": "http://localhost:4000",
		},
	},
});
