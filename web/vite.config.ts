import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

const serverUrl = "http://localhost:4000";

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			"/api": serverUrl,
			"/blob": serverUrl,
		},
	},
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"],
	},
	build: {
		cssMinify: "lightningcss",
	},
});
