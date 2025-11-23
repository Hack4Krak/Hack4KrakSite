import { expect, test } from '@nuxt/test-utils/playwright'

test('error page takes full viewport height on desktop', async ({ page }) => {
  // Navigate to a non-existent page to trigger 404
  await page.goto('/non-existent-page-12345', { waitUntil: 'networkidle' })

  // Verify error content is displayed
  await expect(page.getByText('404')).toBeVisible()

  // Get the page dimensions
  const viewportHeight = page.viewportSize()?.height || 0
  const bodyHeight = await page.evaluate(() => document.body.offsetHeight)

  // Body should take at least the full viewport height
  expect(bodyHeight).toBeGreaterThanOrEqual(viewportHeight)
})

test('error page maintains full height on different viewports', async ({ page }) => {
  const viewportSizes = [
    { width: 1920, height: 1080, name: 'Desktop FHD' },
    { width: 1366, height: 768, name: 'Laptop' },
    { width: 768, height: 1024, name: 'Tablet' },
    { width: 375, height: 667, name: 'Mobile' },
  ]

  for (const size of viewportSizes) {
    await page.setViewportSize({ width: size.width, height: size.height })
    await page.goto('/non-existent-page-12345', { waitUntil: 'networkidle' })

    const bodyHeight = await page.evaluate(() => document.body.offsetHeight)
    expect(bodyHeight, `${size.name} viewport should take full height`).toBeGreaterThanOrEqual(size.height)
  }
})
