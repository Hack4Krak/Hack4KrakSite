import { test } from '@nuxt/test-utils/playwright'

test('test', async ({ page }) => {
  await page.goto('http://localhost:3000/')
  await page.close()
})
