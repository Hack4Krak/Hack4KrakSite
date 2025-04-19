import { expect, test } from '@playwright/test'

test('test', async ({ page }) => {
  await page.goto('http://localhost:3000/login')

  // Verify if email validation works
  await page.getByRole('textbox', { name: 'Email' }).click()
  await page.getByRole('textbox', { name: 'Email' }).fill('meow')
  await page.getByRole('textbox', { name: 'Email' }).press('Tab')
  await expect(page.getByRole('textbox', { name: 'Email' })).toHaveValue('meow')
  await expect(page.locator('text=Niepoprawny adres e-mail')).toBeVisible()

  // Verify if field validation works
  await page.getByRole('textbox', { name: 'Email' }).click()
  await page.getByRole('textbox', { name: 'Email' }).fill('example@gmail.com')
  await page.getByRole('button', { name: 'Zaloguj' }).click()
  await expect(page.locator('#v-0-0-2-2-error')).toContainText('Hasło jest wymagane')

  // Verify if link to register works
  await page.getByRole('link', { name: 'Załóż je' }).click()
  await expect(page.getByRole('heading')).toContainText('Zarejestruj się')
})
