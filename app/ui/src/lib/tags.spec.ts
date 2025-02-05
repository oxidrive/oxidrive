import { describe, expect, it } from "vitest";
import { concat, split } from "./tags";

describe("split", () => {
	it("splits a key-only tag", () => {
		const tag = split("test");

		expect(tag.key).toBe("test");
		expect(tag.value).toBeNull();
	});

	it("splits a key-value tag", () => {
		const tag = split("hello:world");

		expect(tag.key).toBe("hello");
		expect(tag.value).toBe("world");
	});

	it("splits a key-value tag with spaces", () => {
		const tag = split("test:hello world");

		expect(tag.key).toBe("test");
		expect(tag.value).toBe("hello world");
	});

	it("splits a key-value tag with colons", () => {
		const tag = split("test:hello:world");

		expect(tag.key).toBe("test");
		expect(tag.value).toBe("hello:world");
	});
});

describe("concat", () => {
	it("concatenates a key-only tag", () => {
		const tag = concat({ key: "test" });

		expect(tag).toBe("test");
	});

	it("concatenates a key-value tag", () => {
		const tag = concat({ key: "hello", value: "world" });

		expect(tag).toBe("hello:world");
	});

	it("concatenates a key-value tag with spaces", () => {
		const tag = concat({ key: "test", value: "hello world" });

		expect(tag).toBe("test:hello world");
	});

	it("concatenates a key-value tag with colons", () => {
		const tag = concat({ key: "test", value: "hello:world" });

		expect(tag).toBe("test:hello:world");
	});
});
