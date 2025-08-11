import { expect, test } from '@playwright/test'

test('authFormValidation', async ({ page }) => {
  await page.goto('/login', { waitUntil: 'networkidle' })

  // Verify if email validation works
  await page.getByRole('textbox', { name: 'Hasło' }).fill('1')
  await page.getByRole('button', { name: 'Zaloguj', exact: true }).click()
  await expect(page.locator('text=Hasło musi mieć co najmniej 8 znaków')).toBeVisible()
  await expect(page.locator('text=Niepoprawny adres e-mail')).toBeVisible()

  // Verify if link to register works
  await page.getByRole('link', { name: 'Załóż je' }).click()
  await expect(page.getByRole('heading')).toContainText('Zarejestruj się')
})
