import { expect, test } from '@nuxt/test-utils/playwright'

test('UModal component displays properly on error page', async ({ page }) => {
  // Step 1: Navigate to a non-existent page to trigger error
  await page.goto('/asdbahgwaghdva', { waitUntil: 'networkidle' })

  // Step 2: Verify we're on the error screen
  await expect(page.locator('h1')).toContainText('404')
  await expect(page.locator('h2')).toContainText('Uwaga rycerzu')

  // Step 3: Find and click the "Więcej informacji..." button
  const moreInfoButton = page.locator('button', { hasText: 'Więcej informacji...' })
  await expect(moreInfoButton).toBeVisible()
  await moreInfoButton.click()

  // Step 4: Verify the modal shows up
  const modal = page.locator('[role="dialog"]')
  await expect(modal).toBeVisible()

  // Verify modal title
  await expect(modal.locator('h2', {hasText: 'Więcej informacji o błędzie:'})).toBeVisible();

  // Step 5: Check modal inner dimensions (excluding padding)
  const modalContent = modal.locator('[data-testid="modal-content"], .modal-content, .p-4, .p-6').first()

  // If no specific content selector, use the modal body
  const modalBody = modal.locator('section.flex.flex-col.text-lg.space-y-5')
  await expect(modalBody).toBeVisible()

  // Get the bounding box of the modal content area
  const contentBox = await modalBody.boundingBox()
  expect(contentBox).not.toBeNull()

  if (contentBox) {
    // Verify the modal has reasonable dimensions
    // Inner width should be at least 200px and height at least 100px
    expect(contentBox.width).toBeGreaterThan(200)
    expect(contentBox.height).toBeGreaterThan(100)
    
    // Verify the modal content is within reasonable bounds (not too large)
    const viewport = page.viewportSize()
    if (viewport) {
      expect(contentBox.width).toBeLessThan(viewport.width)
      expect(contentBox.height).toBeLessThan(viewport.height)
    }
  }

  // Verify modal contains expected error information sections
  await expect(modalBody.locator('h2', { hasText: 'Kod:' })).toBeVisible()
  await expect(modalBody.locator('h2', { hasText: 'Wiadomość:' })).toBeVisible()
  await expect(modalBody.locator('h2', { hasText: 'Dane:' })).toBeVisible()

  // Verify error code is displayed correctly
  await expect(modalBody.locator('pre').first()).toContainText('404')

  // Verify modal can be closed (if there's a close button)
  const closeButton = modal.locator('button[aria-label="Close"], [data-testid="close-button"], .close-button')
  if (await closeButton.count() > 0) {
    await closeButton.click()
    await expect(modal).not.toBeVisible()
  }
})

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

test('UModal content structure is correct', async ({ page }) => {
  await page.goto('/nonexistentpage123', { waitUntil: 'networkidle' })
  
  const moreInfoButton = page.locator('button', { hasText: 'Więcej informacji...' })
  await moreInfoButton.click()
  
  const modal = page.locator('[role="dialog"]')
  await expect(modal).toBeVisible()
  
  const modalBody = modal.locator('section.flex.flex-col.text-lg.space-y-5')
  
  // Check the structure: should have 3 sections (Kod, Wiadomość, Dane)
  const sections = modalBody.locator('div')
  await expect(sections).toHaveCount(3)
  
  // Check each section has the expected structure
  for (let i = 0; i < 3; i++) {
    const section = sections.nth(i)
    await expect(section.locator('h2')).toBeVisible()
    await expect(section.locator('pre')).toBeVisible()
  }
  
  // Verify the spacing between sections
  const sectionBoxes = await sections.all()
  expect(sectionBoxes.length).toBe(3)
  
  // Each section should have reasonable height
  for (const section of sectionBoxes) {
    const box = await section.boundingBox()
    if (box) {
      expect(box.height).toBeGreaterThan(30) // At least 30px for h2 + pre elements
    }
  }
})
