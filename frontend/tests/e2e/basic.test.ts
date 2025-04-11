import { expect, test } from '@nuxt/test-utils/playwright'

test('Webpage content loading', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByRole('heading')).toContainText('Hack4Krak')
})
