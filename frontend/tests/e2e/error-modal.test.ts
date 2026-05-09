import { expect, test } from '@nuxt/test-utils/playwright'

test('UModal component dimensions are correct on different viewport sizes', async ({ page }) => {
  async function openErrorDetailsModal() {
    await page.getByRole('button', { name: 'Więcej informacji...' }).click()

    const modal = page.getByRole('dialog')
    await expect(modal).toBeVisible()
    await expect(modal.getByText('Szczegóły błędu')).toBeVisible()
    await expect(modal.locator('pre')).toBeVisible()

    return modal
  }

  // Test on desktop viewport
  await page.setViewportSize({ width: 1280, height: 720 })

  await page.goto('/asdbahgwaghdva', { waitUntil: 'networkidle' })

  const desktopModal = await openErrorDetailsModal()
  const desktopBox = await desktopModal.boundingBox()
  expect(desktopBox).not.toBeNull()

  if (desktopBox) {
    expect(desktopBox.width).toBeGreaterThan(300)
    expect(desktopBox.height).toBeGreaterThan(150)
  }

  // Test on mobile viewport
  await page.setViewportSize({ width: 375, height: 667 })

  // Refresh to apply mobile layout
  await page.reload({ waitUntil: 'networkidle' })

  const mobileModal = await openErrorDetailsModal()
  const mobileBox = await mobileModal.boundingBox()
  expect(mobileBox).not.toBeNull()

  if (mobileBox) {
    // On mobile, modal should still be usable but may be smaller
    expect(mobileBox.width).toBeGreaterThan(200)
    expect(mobileBox.height).toBeGreaterThan(100)

    // Modal should not exceed viewport width (with some margin for padding)
    expect(mobileBox.width).toBeLessThanOrEqual(375)
  }
})
