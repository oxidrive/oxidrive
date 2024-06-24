import { type Page, expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test(
	"should not have any automatically detectable accessibility issues",
	testAccessibility("/files"),
);

test("should upload, preview, and download a file", async ({ page }) => {
	const name = `hello${Math.random().toString().substring(2)}.txt`;
	const content = "hello world!";

	await page.goto("/files");
	await page.waitForLoadState("networkidle");
	await expect(page).toHaveURL("/files");

	await test.step("Upload file", upload({ page, name, content }));
	await test.step("Preview file", preview({ page, name, content }));
	await test.step("Download file", download({ page, name, content }));
});

function upload({ page, name, content }: StepArgs): Step {
	return async () => {
		await expect(page.getByText(name)).not.toBeVisible();

		const filechooserPromise = page.waitForEvent("filechooser");

		await page.getByTitle("Upload").click();

		const filechooser = await filechooserPromise;
		await filechooser.setFiles({
			name,
			mimeType: "text/plain",
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

		const file = page.getByTitle(name, { exact: true });
		await expect(file).toBeVisible();
	};
}

function preview({ page, name, content }: StepArgs): Step {
	return async () => {
		await page.getByTitle(name, { exact: true }).click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).toBeVisible();

		const preview = page.frameLocator('iframe[title="Preview"]');
		await expect(preview.locator("body")).toBeVisible();
		await expect(preview.getByText(content)).toBeVisible();

		await page.getByTitle("Close preview").click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).not.toBeVisible();
		await expect(
			page.frameLocator('iframe[title="Preview"]').locator("body"),
		).not.toBeVisible();
	};
}

function download({ page, name }: StepArgs): Step {
	return async () => {
		const actions = page.getByTitle(name, { exact: true }).getByRole("button");

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

type Step = () => Promise<void>;
