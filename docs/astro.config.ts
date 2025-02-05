import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import starlightOpenAPI, { openAPISidebarGroups } from "starlight-openapi";

// biome-ignore lint/suspicious/noExplicitAny: Starlight does not provide a public type for a single Head element
function icon(name: string, media: string): any {
	return {
		tag: "link",
		attrs: {
			rel: "icon",
			href: `/images/${name}`,
			type: "image/svg+xml",
			media,
		},
	};
}

const github = "https://github.com/oxidrive/oxidrive";

export default defineConfig({
	site: "https://oxidrive.github.io",
	base: "oxidrive",
	integrations: [
		starlight({
			title: "Oxidrive",
			logo: {
				light: "./public/images/logo-wide.svg",
				dark: "./public/images/logo-wide-light.svg",
				replacesTitle: true,
			},
			favicon: "/images/logo.svg",
			head: [
				icon("logo.svg", "(prefer-color-scheme:no-preference)"),
				icon("logo.svg", "(prefer-color-scheme:light)"),
				icon("logo-white.svg", "(prefer-color-scheme:dark)"),
			],
			customCss: ["./src/styles/custom.css"],
			social: {
				github,
			},
			sidebar: [
				{
					label: "Overview",
					autogenerate: { directory: "overview" },
				},
				{
					label: "Install",
					autogenerate: { directory: "install" },
				},
				{
					label: "User Guides",
					autogenerate: { directory: "user-guides" },
				},
				{
					label: "Administration",
					autogenerate: { directory: "administration" },
				},
				{
					label: "Reference",
					autogenerate: { directory: "reference" },
				},
				...openAPISidebarGroups,
			],
			defaultLocale: "root",
			locales: {
				root: {
					label: "English",
					lang: "en",
				},
			},
			editLink: {
				baseUrl: `${github}/edit/main/docs/`,
			},
			components: {
				SiteTitle: "./src/components/SiteTitle.astro",
			},
			plugins: [
				starlightOpenAPI([
					{
						base: "api",
						label: "Oxidrive API",
						schema: "../openapi.json",
					},
				]),
			],
		}),
	],
	vite: {
		resolve: {
			preserveSymlinks: true,
		},
	},
});
