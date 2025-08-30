import { expect, test } from '@nuxt/test-utils/playwright'

test('UModal component dimensions are correct on different viewport sizes', async ({ page }) => {
  // Test on desktop viewport
  await page.setViewportSize({ width: 1280, height: 720 })

  await page.goto('/asdbahgwaghdva', { waitUntil: 'networkidle' })

  const moreInfoButton = page.locator('button', { hasText: 'Więcej informacji...' })
  await moreInfoButton.click()

  const modal = page.locator('[role="dialog"]')
  await expect(modal).toBeVisible()

  const modalBody = modal.locator('section.flex.flex-col.text-lg.space-y-5')
  const desktopBox = await modalBody.boundingBox()
  expect(desktopBox).not.toBeNull()

  if (desktopBox) {
    expect(desktopBox.width).toBeGreaterThan(300)
    expect(desktopBox.height).toBeGreaterThan(150)
  }

  // Test on mobile viewport
  await page.setViewportSize({ width: 375, height: 667 })

  // Refresh to apply mobile layout
  await page.reload({ waitUntil: 'networkidle' })

  await moreInfoButton.click()
  await expect(modal).toBeVisible()

  const mobileBox = await modalBody.boundingBox()
  expect(mobileBox).not.toBeNull()

  if (mobileBox) {
    // On mobile, modal should still be usable but may be smaller
    expect(mobileBox.width).toBeGreaterThan(200)
    expect(mobileBox.height).toBeGreaterThan(100)

    // Modal should not exceed viewport width (with some margin for padding)
    expect(mobileBox.width).toBeLessThan(375 - 40) // 40px margin for padding/borders
  }
})
