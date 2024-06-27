import { type Locator, type Page, expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test(
	"should not have any automatically detectable accessibility issues",
	testAccessibility("/files"),
);

test("should upload, preview, and download a file", async ({ page }) => {
	const name = `hello${Math.random().toString().substring(2)}.json`;
	const content = '{"hello":"world!"}';

	await page.goto("/files");
	await page.waitForLoadState("networkidle");
	await expect(page).toHaveURL("/files");

	const file = await test.step("Upload file", upload({ page, name, content }));
	await test.step("Preview file", preview(file, { page, name, content }));
	await test.step("Download file", download(file, { page, name, content }));
});

function upload({ page, name, content }: StepArgs): Step<Locator> {
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
		for (const toast of toasts) {
			await expect(toast).toBeVisible();
			await expect(toast).toHaveAttribute("data-toast-level", "info");
			await expect(toast).toContainText(name);
		}

		await Promise.allSettled(
			toasts.map((toast) => toast.getByTitle("Close toast").click()),
		);

		await expect(page.getByText("No files in here")).not.toBeVisible();

		const file = page.getByTitle(name, { exact: true }).first();
		await expect(file).toBeVisible();
		return file;
	};
}

function preview(file: Locator, { page, name, content }: StepArgs): Step {
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

interface StepArgs {
	page: Page;
	name: string;
	content: string;
}

type Step<T = void> = () => Promise<T>;
