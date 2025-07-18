import { expect, test } from '@nuxt/test-utils/playwright'

test('webpage basic content loading', async ({ page }) => {
  await page.goto('/', { waitUntil: 'networkidle' })

  // Check title
  await expect(page).toHaveTitle('Strona Główna | Hack4Krak CTF')

  // Check favicon
  const favicon = await page.$('link[rel="icon"]')
  const href = await favicon?.getAttribute('href')
  expect(href).toContain('favicon')
})
