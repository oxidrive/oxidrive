import { expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test.describe("uploading a file", () => {
	test("should succeed", async ({ page }) => {
		const name = `hello${Math.random().toString().substring(2)}.txt`;

		await page.goto("/files");
		await expect(page).toHaveURL("/files");

		await expect(page.getByText(name)).not.toBeVisible();

		const filechooserPromise = page.waitForEvent("filechooser");

		await page.getByTitle("Upload").click();

		const filechooser = await filechooserPromise;
		await filechooser.setFiles({
			name,
			mimeType: "text/plain",
			buffer: Buffer.from("hello world!"),
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
		await file.click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).toBeVisible();
		await expect(
			page.frameLocator('iframe[title="Preview"]').locator("body"),
		).toBeVisible();
		await page.getByTitle("Close preview").click();

		await expect(
			page.getByRole("heading", { name, exact: true }),
		).not.toBeVisible();
		await expect(
			page.frameLocator('iframe[title="Preview"]').locator("body"),
		).not.toBeVisible();
	});

	test(
		"should not have any automatically detectable accessibility issues",
		testAccessibility("/files"),
	);
});
