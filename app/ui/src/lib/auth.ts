import { getContext, setContext } from "svelte";
import { type Writable, writable } from "svelte/store";
import type { SchemaAccountInfo } from "./openapi";

const userKey = Symbol.for("auth.user");

export type User = SchemaAccountInfo;

export function getUser(): Writable<User | null> {
	return getContext(userKey);
}

export function setUser(user: User | null) {
	setContext(userKey, writable(user));
}
