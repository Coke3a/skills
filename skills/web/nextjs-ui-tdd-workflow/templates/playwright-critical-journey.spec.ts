import { test, expect } from '@playwright/test'

test('user can complete the ExampleFeature journey', async ({ page }) => {
  await page.goto('/example')

  await expect(page.getByRole('heading', { name: /example/i })).toBeVisible()

  await page.getByRole('button', { name: /start/i }).click()
  await page.getByLabel(/name/i).fill('Example User')
  await page.getByRole('button', { name: /submit/i }).click()

  await expect(page.getByText(/example feature created/i)).toBeVisible()
})
