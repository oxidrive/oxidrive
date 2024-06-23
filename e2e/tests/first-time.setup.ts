import { expect, test } from "@playwright/test";
import { testAccessibility } from "./fixtures";

const authFile = "playwright/.auth/user.json";

test.describe("setup flow", () => {
	test("should be completed to unlock the instance", async ({
		page,
		request,
	}) => {
		const username = "playwright";
		const password = "playwright";

		const status = await request.get("/api/instance");
		expect(status.ok()).toBeTruthy();

		const {
			status: { setupCompleted },
		} = await status.json();

		await page.goto("/setup");
		await page.waitForLoadState("networkidle");

		if (!setupCompleted) {
			await expect(page).toHaveURL("/setup");

			// Normally this should be its own test, however the first time setup flow is a one-shot process
			// that is never accessible again after it has run. Running a11y checks as their own
			// tests would require re-initializing the database manually.
			await testAccessibility("/setup")({ page });

			await expect(
				page.getByRole("heading", { name: "Create an admin account" }),
			).toBeVisible();

			await page.getByPlaceholder("Username").fill(username);
			await page.getByPlaceholder("Password", { exact: true }).fill(password);
			await page.getByPlaceholder("Confirm Password").fill(password);

			await page.getByRole("button", { name: "Complete Setup" }).click();
		}

		await expect(page).toHaveURL("/login?redirect=/files");

		await page.getByPlaceholder("Username").fill(username);
		await page.getByPlaceholder("Password").fill(password);
		await page.getByRole("button", { name: "Sign In" }).click();

		await expect(page).toHaveURL("/files");

		// Persisting the authentication state for the other tests
		await page.context().storageState({ path: authFile });
	});
});
