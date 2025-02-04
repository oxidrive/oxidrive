// biome-ignore lint/suspicious/noExplicitAny: we never look at the arguments here
export function debounce<A extends any[]>(
	target: (...args: A) => void,
	wait: number,
): (...args: A) => void {
	let timer: ReturnType<typeof setTimeout>;
	return (...args) => {
		clearTimeout(timer);

		timer = setTimeout(() => {
			target(...args);
		}, wait);
	};
}
