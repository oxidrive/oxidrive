import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test.describe("setup flow", () => {
	test("should be completed to unlock the instance", async ({ page }) => {
		await page.goto("/");
		await expect(page).toHaveURL("/setup");

		await expect(
			page.getByRole("heading", { name: "Create an admin account" }),
		).toBeVisible();

		await page.getByPlaceholder("Username").fill("playwright");
		await page.getByPlaceholder("Password", { exact: true }).fill("playwright");
		await page.getByPlaceholder("Confirm Password").fill("playwright");

		await page.getByRole("button", { name: "Complete Setup" }).click();

		await expect(page).toHaveURL("/");
		await expect(
			page.getByRole("link", { name: "Join the community!" }),
		).toBeVisible();
	});

	test("should not have any automatically detectable accessibility issues", async ({
		page,
	}) => {
		await page.goto("/setup");
		await expect(page).toHaveURL("/setup");

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	});
});
