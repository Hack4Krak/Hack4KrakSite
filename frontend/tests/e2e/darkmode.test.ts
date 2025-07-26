import { expect, test } from '@nuxt/test-utils/playwright'

test('has class="dark" in SSR response without hydration', async ({ page }) => {
  const response = await page.request.get('/')
  const html = await response.text()
  expect(html).toContain('class="dark"')
})
