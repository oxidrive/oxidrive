import { FluentBundle, FluentResource } from "@fluent/bundle";
import { negotiateLanguages } from "@fluent/langneg";

export function parseLanguageHeader(value: string): string[] {
	return value
		.split(",")
		.flatMap((l) => l.split(";").at(0))
		.filter((l) => typeof l === "string") as string[];
}

export interface Resources {
	[k: string]: FluentResource[];
}

const PREFIX = "../../translations/";

export async function loadResources(): Promise<Resources> {
	const resources: Resources = {};
	const imports = Object.entries(
		import.meta.glob("../../translations/**/*.ftl", {
			query: "?raw",
			import: "default",
		}),
	).map(([filePath, importFn]) => ({
		lang: dirname(filePath.replace(PREFIX, "")),
		importFn,
	}));

	for (const { lang, importFn } of imports) {
		resources[lang] = resources[lang] || [];
		const body = (await importFn()) as string;
		resources[lang].push(new FluentResource(body));
	}

	return resources;
}

export const DEFAULT_LOCALE = "en";

export function generateBundles(
	resources: Resources,
	locales: readonly string[],
	defaultLocale = DEFAULT_LOCALE,
): FluentBundle[] {
	const supportedLocales = Object.keys(resources);

	const selectedLocales = negotiateLanguages(locales, supportedLocales, {
		defaultLocale,
		strategy: "lookup",
	});

	return selectedLocales.map((locale) => {
		const bundle = new FluentBundle(locale);
		for (const resource of resources[locale]) {
			bundle.addResource(resource);
		}
		return bundle;
	});
}

function dirname(path: string): string {
	return path.split("/").at(0) || path;
}
