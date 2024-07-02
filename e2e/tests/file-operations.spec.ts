import { type Locator, type Page, expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test(
	"should not have any automatically detectable accessibility issues",
	testAccessibility("/files"),
);

test("should upload, preview, download, and delete a file", async ({
	page,
}) => {
	const name = `hello${Math.random().toString().substring(2)}.json`;
	const content = '{"hello":"world!"}';

	await page.goto("/files");
	await page.waitForLoadState("networkidle");
	await expect(page).toHaveURL("/files");

	const file = await test.step("Upload file", upload({ page, name, content }));
	await test.step("Preview file", preview(file, { page, name, content }));
	await test.step("Download file", download(file, { page, name, content }));
	await test.step("Delete file", remove(file, { page, name, content }));
});

test("should upload, rename, and delete a file", async ({ page }) => {
	const run = Math.random().toString().substring(2);
	const name = `hello${run}.json`;
	const newName = `changed${run}.json`;
	const content = '{"hello":"world!"}';

	await page.goto("/files");
	await page.waitForLoadState("networkidle");
	await expect(page).toHaveURL("/files");

	const file = await test.step("Upload file", upload({ page, name, content }));
	const renamed = await test.step(
		"Rename file",
		rename(file, newName, { page, name, content }),
	);
	await test.step(
		"Delete file",
		remove(renamed, { page, name: newName, content }),
	);
});

test("should upload, and rename a folder", async ({ page }) => {
	const run = Math.random().toString().substring(2);
	const folderName = `tests${run}`;
	const newFolderName = `changed${run}`;
	const fileName = `hello${run}.json`;
	const name = `${folderName}/${fileName}`;
	const content = '{"hello":"world!"}';

	await page.goto("/files");
	await page.waitForLoadState("networkidle");
	await expect(page).toHaveURL("/files");

	await test.step(
		"Upload file",
		upload({ page, name, content, folder: folderName }),
	);
	const folder = page.getByTitle(folderName, { exact: true }).first();
	await test.step(
		"Rename folder",
		rename(folder, newFolderName, { page, name, content }),
	);

	await page.getByRole("link", { name: newFolderName, exact: true }).click();

	await expect(page).toHaveURL(`/files/${newFolderName}`);

	const file = page.getByTitle(fileName, { exact: true }).first();
	await test.step(
		"Preview file",
		preview(file, { page, name: fileName, content }),
	);
	await test.step(
		"Delete file",
		remove(file, { page, name: fileName, content }),
	);
});

function upload({ page, name, content, folder }: StepArgs): Step<Locator> {
	return async () => {
		await expect(page.getByText(name)).not.toBeVisible();

		const filechooserPromise = page.waitForEvent("filechooser");

		await page.getByRole("button", { name: "Create" }).click();
		await page.getByRole("menuitem", { name: "Upload file" }).click();

		const filechooser = await filechooserPromise;
		await filechooser.setFiles({
			name,
			mimeType: "application/octet-stream",
			buffer: Buffer.from(content),
		});

		await page.waitForLoadState("networkidle");

		const toasts = await page.getByRole("alert").all();

		await Promise.allSettled(
			toasts.map(async (toast) => {
				await expect(toast).toBeVisible();
				await expect(toast).toHaveAttribute("data-toast-level", "info");
				await expect(toast).toContainText(name);
				await toast.getByTitle("Close toast").click();
			}),
		);

		const file = page.getByTitle(folder ?? name, { exact: true }).first();
		await file.scrollIntoViewIfNeeded();
		await expect(file).toBeVisible();
		return file;
	};
}

function preview(file: Locator, { page, name }: StepArgs): Step {
	return async () => {
		await file.click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).toBeVisible();

		const preview = page.locator(".preview");
		await expect(preview).toBeVisible();
		await expect(preview.getByText("cannot be previewed")).toContainText(name);
		const button = preview.getByRole("link", { name: "Download" });
		await expect(button).toBeVisible();

		await page.getByTitle("Close preview").click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).not.toBeVisible();
		await expect(preview).not.toBeVisible();
	};
}

function download(file: Locator, { page, name }: StepArgs): Step {
	return async () => {
		const actions = file.getByTestId("file-actions");

		await expect(actions).toBeVisible();
		await actions.click();

		const downloadPromise = page.waitForEvent("download");

		const action = page.getByRole("menuitem", { name: "Download" });
		await expect(action).toBeVisible();
		await action.click();

		const download = await downloadPromise;
		expect(download.suggestedFilename()).toBe(name);
	};
}

function rename(
	file: Locator,
	newName: string,
	{ page }: StepArgs,
): Step<Locator> {
	return async () => {
		const actions = file.getByTestId("file-actions");

		await expect(actions).toBeVisible();
		await actions.click();

		const action = page.getByRole("menuitem", { name: "Rename" });
		await expect(action).toBeVisible();
		await action.click();

		const input = page.getByRole("textbox");
		await expect(input).toBeVisible();
		await input.fill(newName);
		await page.keyboard.press("Enter");

		const renamed = page.getByTitle(newName, { exact: true }).first();
		await expect(renamed).toBeVisible();
		return renamed;
	};
}

function remove(file: Locator, { page }: StepArgs): Step {
	return async () => {
		page.on("dialog", (dialog) => dialog.accept());

		const actions = file.getByTestId("file-actions");

		await expect(actions).toBeVisible();
		await actions.click();

		const action = page.getByRole("menuitem", { name: "Delete" });
		await expect(action).toBeVisible();
		await action.click();

		await expect(file).not.toBeVisible();
	};
}

interface StepArgs {
	page: Page;
	name: string;
	content: string;
	folder?: string;
}

type Step<T = void> = () => Promise<T>;
