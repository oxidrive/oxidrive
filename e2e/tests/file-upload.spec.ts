import { expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test.describe("uploading a file", () => {
	test("should succeed", async ({ page }) => {
		await page.goto("/files");
		await expect(page).toHaveURL("/files");

		await expect(page.getByText("No files in here")).toBeVisible();

		const responsePromise = page.waitForResponse("/api/files");
		const filechooserPromise = page.waitForEvent("filechooser");

		await page.getByLabel("Upload").click();

		const filechooser = await filechooserPromise;
		await filechooser.setFiles({
			name: "hello.txt",
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
	});

	test(
		"should not have any automatically detectable accessibility issues",
		testAccessibility("/files"),
	);
});
