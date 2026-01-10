import { test, expect } from "@playwright/test";

// Use a stable podcast RSS feed for testing
const TEST_FEED_URL = "https://feeds.simplecast.com/lP7owBq8";

test.describe("Add Podcast Page", () => {
  test("loads successfully", async ({ page }) => {
    await page.goto("/add");
    await expect(page.locator("header .title")).toContainText("Add podcast");
    await expect(page.locator("header .subtitle")).toContainText(
      "From an RSS feed URL"
    );
  });

  test("has slug and feed URL fields", async ({ page }) => {
    await page.goto("/add");
    await expect(page.getByText("Slug", { exact: true })).toBeVisible();
    await expect(page.getByText("Feed URL", { exact: true })).toBeVisible();
    await expect(page.locator("input")).toHaveCount(2);
  });

  test("has submit button", async ({ page }) => {
    await page.goto("/add");
    const submitButton = page.getByRole("button", { name: "Add Podcast" });
    await expect(submitButton).toBeVisible();
  });

  test("submit button is disabled when fields are empty", async ({ page }) => {
    await page.goto("/add");
    const submitButton = page.getByRole("button", { name: "Add Podcast" });
    await expect(submitButton).toBeDisabled();
  });

  test("slug field accepts input", async ({ page }) => {
    await page.goto("/add");
    const slugInput = page.locator("input").first();
    await slugInput.fill("my-podcast");
    await expect(slugInput).toHaveValue("my-podcast");
  });

  test("url field accepts input", async ({ page }) => {
    await page.goto("/add");
    const urlInput = page.locator("input").nth(1);
    await urlInput.fill("https://example.com/feed.xml");
    await expect(urlInput).toHaveValue("https://example.com/feed.xml");
  });

  test("has back navigation to index", async ({ page }) => {
    await page.goto("/add");
    const backButton = page.locator("header a").first();
    await backButton.click();
    await expect(page).toHaveURL("/");
  });

  test("can navigate via navbar", async ({ page }) => {
    await page.goto("/");
    await page.locator("footer").getByText("Add Podcast").click();
    await expect(page).toHaveURL("/add");
  });

  test("successfully adds podcast and shows on index", async ({ page }) => {
    // Generate unique slug for this test run
    const testSlug = `test-podcast-${Date.now()}`;

    // Add a podcast
    await page.goto("/add");

    // Wait for any hot-reload to complete
    await page.waitForTimeout(2000);

    const slugInput = page.locator("input").first();
    const urlInput = page.locator("input").nth(1);

    // Fill slug and wait
    await slugInput.fill(testSlug);
    await page.waitForTimeout(100);

    // Fill URL and wait
    await urlInput.fill(TEST_FEED_URL);
    await page.waitForTimeout(100);

    // Wait for validation to enable submit button
    const submitButton = page.getByRole("button", { name: "Add Podcast" });
    await expect(submitButton).toBeEnabled({ timeout: 10000 });

    // Submit the form
    await submitButton.click();

    // Should navigate to the podcast page after success
    await expect(page).toHaveURL(`/podcasts/${testSlug}`, { timeout: 30000 });

    // Navigate to index and verify podcast appears
    await page.goto("/");
    await expect(page.getByText(testSlug)).toBeVisible({ timeout: 10000 });
  });
});
