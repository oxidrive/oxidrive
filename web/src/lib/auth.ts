import { getContext, setContext } from "svelte";
import { type Writable, writable } from "svelte/store";

const userKey = Symbol.for("auth.user");

export interface User {
	id: string;
	username: string;
}

export function getUser(): Writable<User | null> {
	return getContext(userKey);
}

export function initUser(user: User | null) {
	setContext(userKey, writable(user));
}
