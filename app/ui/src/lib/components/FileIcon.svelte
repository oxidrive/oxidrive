<script lang="ts">
import type { SchemaFileData } from "$lib/openapi";
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
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

// Tell Font Awesome to skip adding the CSS automatically since it's being imported above
fontawesome.autoAddCss = false;

interface Props {
	file: SchemaFileData;
	height?: `${number}px`;
	width?: `${number}px`;
}

let { file, height = "40px", width = "40px" }: Props = $props();

let color = "primary-900" as const;

/*
    This man-made horror beyond human comprehension is necessary because FontAwesomeIcon is bugged
    and injects the style object as a string, result in an HTML prop with value `style="[object Object]"`,
    which of course is invalid. Since we can't ignore type errors in Svelte component with @ts-ignore yet,
    we have to do this ungodly abomination instead. I hate the JS ecosystem.
  */
const style = `height: ${height}; width: ${width};` as unknown as object;

function extractContentType(mime: string): string {
	return mime.split(";")[0];
}

function icon(): IconDefinition {
	const mime = extractContentType(file.content_type);

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
