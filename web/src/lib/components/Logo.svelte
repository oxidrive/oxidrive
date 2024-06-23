<script lang="ts">
import logoWhiteWithName from "$lib/assets/logo-white-with-name.svg";
import logoWhite from "$lib/assets/logo-white.svg";
import logoPrimaryWithName from "$lib/assets/logo-with-name.svg";
import logoPrimary from "$lib/assets/logo.svg";

export let color: "primary" | "white";
export let name = false;

function load(): string {
	switch (true) {
		case color === "white" && name:
			return logoWhiteWithName;
		case color === "white" && !name:
			return logoWhite;
		case color === "primary" && name:
			return logoPrimaryWithName;
		default:
			return logoPrimary;
	}
}

const src = load();
</script>

<img {src} alt="Oxidrive" />
