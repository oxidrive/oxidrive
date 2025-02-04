import { addToast, reportError } from "$lib/components/Toast.svelte";
import type { SvelteFluent } from "@nubolab-ffwd/svelte-fluent";
import type { SchemaApiError } from "./openapi";

export async function uploadFile(
	file: File,
	localize: SvelteFluent["localize"],
): Promise<boolean> {
	addToast({
		data: {
			title: localize("files-upload-started", {
				file: file.name,
			}),
			message: localize("files-upload-started.message"),
			level: "info",
		},
	});
	const form = new FormData();

	if (file.type === "application/json") {
		form.append("file", await escapeJSONFile(file));
	} else {
		form.append("file", file);
	}

	// can't use openapi-fetch here because it doesn't handle
	// file uploads / multipart form data very well
	const response = await fetch("/files", {
		method: "POST",
		body: form,
	});

	if (!response.ok) {
		try {
			const error: SchemaApiError = await response.json();
			reportError(error, localize("files-upload-failed", { file: file.name }));
		} catch (e) {
			const err = e as Error;

			reportError(
				{ error: "JSON_RESPONSE_ERROR", message: err.message },
				localize("files-upload-failed", { file: file.name }),
			);
		}
		return false;
	}

	addToast({
		data: {
			title: localize("files-upload-succeeded", {
				file: file.name,
			}),
			message: localize("files-upload-succeeded.message"),
			level: "info",
		},
	});

	return true;
}

async function escapeJSONFile(file: globalThis.File): Promise<globalThis.File> {
	const json = await new Promise<string>((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsText(file, "UTF-8");
		reader.addEventListener("load", () => {
			resolve(reader.result as string);
		});
		reader.addEventListener("error", reject);
	});

	return new globalThis.File([json], file.name, { ...file });
}
