import { expect, test } from '@nuxt/test-utils/playwright'
import { devices } from '@playwright/test'

/**
 * Regression test to verify that the navbar remains visible in the viewport
 * when the page is scrolled to the bottom on mobile devices.
 *
 * This test:
 * 1. Sets up a mobile viewport (iPhone 12)
 * 2. Navigates to the homepage
 * 3. Verifies we're in mobile view
 * 4. Scrolls to the bottom of the page
 * 5. Verifies the navbar and mobile menu toggle are still visible
 * 6. Checks that the navbar is within the viewport boundaries
 */
test('navbar visibility on mobile when scrolled to bottom', async ({ page }) => {
  // Use iPhone 12 viewport
  await page.setViewportSize(devices['iPhone 12'].viewport)

  // Navigate to homepage
  await page.goto('/', { waitUntil: 'networkidle' })

  // Get the mobile navbar element - container with sticky positioning that contains the mobile navigation
  const navbar = page.locator('.sticky.top-0')

  // Verify we're using the mobile view
  await expect(page.locator('.md\\:hidden')).toBeVisible()
  await expect(page.locator('.hidden.md\\:block')).not.toBeVisible()

  // Verify navbar is initially visible
  await expect(navbar).toBeVisible()

  // Scroll to the bottom of the page
  await page.evaluate(() => {
    window.scrollTo(0, document.body.scrollHeight)
  })

  // Wait for any animations to complete
  await page.waitForTimeout(500)

  // Verify navbar is still visible after scrolling
  await expect(navbar).toBeVisible()

  // Verify the mobile menu toggle button is visible
  const mobileMenuToggle = page.locator('[data-testid="mobile-menu-toggle"]')
  await expect(mobileMenuToggle).toBeVisible()

  // Additional check: verify navbar is in viewport
  // Check if the navbar's bounding box is within the viewport
  const navbarBox = await navbar.boundingBox()
  const viewportSize = page.viewportSize()

  expect(navbarBox).not.toBeNull()
  expect(viewportSize).not.toBeNull()

  if (navbarBox && viewportSize) {
    // Check if the top of the navbar is visible (within viewport)
    expect(navbarBox.y).toBeGreaterThanOrEqual(0)
    expect(navbarBox.y).toBeLessThanOrEqual(viewportSize.height)
  }
})
