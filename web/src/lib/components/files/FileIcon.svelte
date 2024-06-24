<script lang="ts">
import type { File } from "$lib/api";
import {
	type IconDefinition,
	config as fontawesome,
} from "@fortawesome/fontawesome-svg-core";
import "@fortawesome/fontawesome-svg-core/styles.css";
import {
	faFile,
	faFileCode,
	faFileImage,
	faFilePdf,
	faFileVideo,
	faFolder,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

// Tell Font Awesome to skip adding the CSS automatically since it's being imported above
fontawesome.autoAddCss = false;

export let file: File;
export let height: `${number}px` = "40px";
export let width: `${number}px` = "40px";

let color: "primary-500" | "primary-900" =
	file.type === "folder" ? "primary-500" : "primary-900";

/*
    This man-made horrors beyond human comprehension is necessary because FontAwesomeIcon is bugged
    and injects the style object as a string, result in an HTML prop with value `style="[object Object]"`,
    which of course is invalid. Since we can't ignore type errors in Svelte component with @ts-ignore yet [1],
    we have to do this ungodly abomination instead. I hate the JS ecosystem.
  */
const style = `height: ${height}; width: ${width};` as unknown as object;

function extractContentType(mime: string): string {
	return mime.split(";")[0];
}

function icon(): IconDefinition {
	if (file.type === "folder") {
		return faFolder;
	}

	const mime = extractContentType(file.contentType);

	if (mime.startsWith("image/")) {
		return faFileImage;
	}

	if (mime.startsWith("video/")) {
		return faFileVideo;
	}

	switch (mime) {
		case "application/pdf":
			return faFilePdf;
		case "application/json":
		case "application/xml":
			return faFileCode;
		default:
			return faFile;
	}
}
</script>

<FontAwesomeIcon icon={icon()} {style} class={`text-${color}`} />
