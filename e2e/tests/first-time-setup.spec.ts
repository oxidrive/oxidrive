import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

test.describe("setup flow", () => {
	test("should be completed to unlock the instance", async ({ page }) => {
		await page.goto("/");
		await page.waitForLoadState("networkidle");

		await expect(page).toHaveURL("/setup");

		// Normally this should be its own test, however the first time setup flow is a one-shot process
		// that is never accessible again after it has run. Running a11y checks as their own
		// tests would require re-initializing the database manually.
		await testAccessibility("/setup")({ page });

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
});
