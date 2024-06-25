import { defineConfig, devices } from "@playwright/test";

const authFile = "playwright/.auth/user.json";
const baseURL = "http://localhost:4000";

function minutes(min: number): number {
	return min * 60 * 1000;
}

const projects: { name: string; device: string }[] = [
	{ name: "chromium", device: "Desktop Chrome" },
	{ name: "firefox", device: "Desktop Firefox" },
	{ name: "webkit", device: "Desktop Safari" },
	{ name: "chromium-mobile", device: "Pixel 5" },
	{ name: "webkit-mobile", device: "iPhone 12" },
];
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

	projects: projects.flatMap(({ name, device }) => [
		{
			name: `setup-${name}`,
			use: { ...devices[device] },
			testMatch: /.*\.setup\.ts/,
		},
		{
			name,
			use: { ...devices[device], storageState: authFile },
			dependencies: [`setup-${name}`],
		},
	]),

	webServer: {
		command: "just start",
		url: baseURL,
		reuseExistingServer: !process.env.CI,
		timeout: minutes(5),
		stdout: "pipe",
		stderr: "pipe",
	},
});
