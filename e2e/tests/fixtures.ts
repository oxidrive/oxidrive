import AxeBuilder from "@axe-core/playwright";
import { type Page, expect } from "@playwright/test";

export type TestFunction = (args: { page: Page }) => Promise<void>;

export function testAccessibility(path: `/${string}`): TestFunction {
	return async ({ page }) => {
		await page.goto(path);
		await expect(page).toHaveURL(path);

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	};
}
