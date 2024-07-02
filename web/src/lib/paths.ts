export function rename(path: string, name: string): string {
	const segments = path.split("/");

	const oldName = segments.pop();
	if (!oldName) return path;

	return [...segments, name].join("/");
}
