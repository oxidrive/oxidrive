import type { SchemaFileData, SchemaTag } from "$lib/openapi";

export const SYSTEM_TAGS = ["name", "content_type", "size", "ext"];

export function excludeSystemTags(tag: SchemaTag | string): boolean {
	const key = typeof tag === "string" ? tag : tag.key;
	return !SYSTEM_TAGS.includes(key);
}

export function userTags(file: SchemaFileData): SchemaTag[] {
	const tags = [...file.tags];
	tags.sort();
	return tags.filter(excludeSystemTags);
}

export function concat(tag: SchemaTag | string): string {
	if (typeof tag === "string") return tag;

	return tag.value ? `${tag.key}:${tag.value}` : tag.key;
}

export function split(tag: SchemaTag | string): SchemaTag {
	if (typeof tag !== "string") return tag;

	const [key, value] = tag.split(":");
	return { key, value };
}

export function equals(
	tag: { key: string } | SchemaTag,
): (other: { key: string } | SchemaTag) => boolean {
	return (other) => {
		const key = tag.key === other.key;

		let value = true;

		if ("value" in tag && "value" in other) {
			value = tag.value === other.value;
		}

		return key && value;
	};
}

export function not(
	// biome-ignore lint/suspicious/noExplicitAny: we really don't care about this value
	predicate: (...args: any[]) => boolean,
	// biome-ignore lint/suspicious/noExplicitAny: we really don't care about this value
): (...args: any[]) => boolean {
	return (...args) => !predicate(...args);
}
