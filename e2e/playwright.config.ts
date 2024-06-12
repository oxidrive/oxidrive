import { defineConfig, devices } from "@playwright/test";

const authFile = "playwright/.auth/user.json";
const baseURL = "http://localhost:4000";

function minutes(min: number): number {
	return min * 60 * 1000;
}

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// require('dotenv').config();

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
	testDir: "./tests",
	fullyParallel: true,
	forbidOnly: !!process.env.CI,
	retries: process.env.CI ? 2 : 0,
	workers: process.env.CI ? 1 : undefined,
	reporter: [["list"], ["html", { open: "never" }]],
	use: {
		baseURL,
		trace: "retain-on-failure",
	},

	/* Configure projects for major browsers */
	projects: [
		{ name: "setup", testMatch: /.*\.setup\.ts/ },
		{
			name: "chromium",
			use: { ...devices["Desktop Chrome"], storageState: authFile },
			dependencies: ["setup"],
		},

		{
			name: "firefox",
			use: { ...devices["Desktop Firefox"], storageState: authFile },
			dependencies: ["setup"],
		},

		{
			name: "webkit",
			use: { ...devices["Desktop Safari"], storageState: authFile },
			dependencies: ["setup"],
		},

		/* Test against mobile viewports. */
		{
			name: "Mobile Chrome",
			use: { ...devices["Pixel 5"], storageState: authFile },
			dependencies: ["setup"],
		},
		{
			name: "Mobile Safari",
			use: { ...devices["iPhone 12"], storageState: authFile },
			dependencies: ["setup"],
		},

		/* Test against branded browsers. */
		// {
		//   name: 'Microsoft Edge',
		//   use: { ...devices['Desktop Edge'], channel: 'msedge' },
		// },
		// {
		//   name: 'Google Chrome',
		//   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
		// },
	],

	webServer: {
		command: "just start",
		url: baseURL,
		reuseExistingServer: !process.env.CI,
		timeout: minutes(5),
		stdout: "pipe",
		stderr: "pipe",
	},
});
