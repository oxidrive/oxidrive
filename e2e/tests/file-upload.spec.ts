import { expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test.describe("uploading a file", () => {
	test("should succeed", async ({ page }) => {
		const name = `hello${Math.random().toString().substring(2)}.txt`;

		await page.goto("/files");
		await expect(page).toHaveURL("/files");

		await expect(page.getByText(name)).not.toBeVisible();

		const responsePromise = page.waitForResponse("/api/files");
		const filechooserPromise = page.waitForEvent("filechooser");

		await page.getByLabel("Upload").click();

		const filechooser = await filechooserPromise;
		await filechooser.setFiles({
			name,
			mimeType: "text/plain",
			buffer: Buffer.from("hello world!"),
		});

		// TODO: replace with "check the uploaded file appears in the page" once we implement file listing (https://github.com/oxidrive/oxidrive/issues/4)
		const response = await responsePromise;
		expect(response.ok()).toBeTruthy();

		const { id, ok } = await response.json();
		await expect(id).toEqual(
			expect.stringMatching(
				/^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$/,
			),
		);
		expect(ok).toBeTruthy();

		const toast = page.getByTestId("toast-0");
		await expect(toast).toBeVisible();
		await expect(toast).toHaveAttribute("data-toast-level", "info");
		await expect(toast).toContainText(name);

		await page.reload();

		await expect(page.getByText("No files in here")).not.toBeVisible();
		await expect(page.getByText(name)).toBeVisible();
	});

	test(
		"should not have any automatically detectable accessibility issues",
		testAccessibility("/files"),
	);
});
