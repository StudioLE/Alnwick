import { defineConfig, devices } from "@playwright/test";

/** Timeouts in milliseconds, shared across all test files. */
export const TIMEOUTS = {
  /** Wait for WASM hydration to complete before interacting with the page. */
  hydration: 2000,
  /** Short wait for client-side UI updates such as validation or navigation. */
  ui: 3000,
};

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:8080",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  webServer: process.env.CI
    ? undefined
    : {
        command: "cd ../docker/dioxus && docker compose up dev",
        url: "http://localhost:8080",
        reuseExistingServer: true,
      },
});
