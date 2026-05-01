import { mountSuspended } from '@nuxt/test-utils/runtime'
import { flushPromises } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import SubmitPersonalInfoPage from '@/pages/account/submit_personal_info.vue'

vi.mock('@/composables/useAuth', () => ({
  useAuth: vi.fn().mockResolvedValue({ data: { value: null } }),
}))

async function mountPage() {
  return mountSuspended(SubmitPersonalInfoPage)
}

async function clickButton(wrapper: Awaited<ReturnType<typeof mountPage>>, text: string) {
  const btn = wrapper.findAll('button').find(b => b.text().includes(text))
  await btn!.trigger('click')
  await flushPromises()
}

async function navigateToAboutStep(wrapper: Awaited<ReturnType<typeof mountPage>>) {
  await clickButton(wrapper, 'Zaczynajmy')
}

async function navigateToSourceStep(wrapper: Awaited<ReturnType<typeof mountPage>>) {
  await navigateToAboutStep(wrapper)
  await wrapper.find('input[placeholder="Twoje imię"]').setValue('Jan')
  await wrapper.find('input[placeholder="np. Kraków"]').setValue('Kraków')
  await flushPromises()
  await clickButton(wrapper, 'Dalej')
}

async function navigateToConsentsStep(wrapper: Awaited<ReturnType<typeof mountPage>>) {
  await navigateToSourceStep(wrapper)
  await wrapper.findAll('button').find(b => b.text().includes('Instagram'))!.trigger('click')
  await flushPromises()
  await clickButton(wrapper, 'Dalej')
}

describe('SubmitPersonalInfoPage', () => {
  it('renders welcome step by default', async () => {
    const wrapper = await mountPage()
    expect(wrapper.text()).toContain('Krok 1 z 4')
    expect(wrapper.findAll('button').find(b => b.text().includes('Zaczynajmy'))).toBeTruthy()
  })

  it('does not show "Wstecz" button on welcome step', async () => {
    const wrapper = await mountPage()
    expect(wrapper.findAll('button').find(b => b.text().includes('Wstecz'))).toBeUndefined()
  })

  it('advances to about step from welcome', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    expect(wrapper.text()).toContain('Krok 2 z 4')
    expect(wrapper.text()).toContain('O Tobie')
  })

  it('shows "Wstecz" button from step 2 onwards', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    expect(wrapper.findAll('button').find(b => b.text().includes('Wstecz'))).toBeTruthy()
  })

  it('does not advance from about step without name and location', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    await clickButton(wrapper, 'Dalej')
    expect(wrapper.text()).toContain('Krok 2 z 4')
  })

  it('does not advance from about step with only name filled', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    await wrapper.find('input[placeholder="Twoje imię"]').setValue('Jan')
    await flushPromises()
    await clickButton(wrapper, 'Dalej')
    expect(wrapper.text()).toContain('Krok 2 z 4')
  })

  it('advances to source step after filling name and location', async () => {
    const wrapper = await mountPage()
    await navigateToSourceStep(wrapper)
    expect(wrapper.text()).toContain('Krok 3 z 4')
    expect(wrapper.text()).toContain('Jak nas znalazłeś?')
  })

  it('renders all 8 referral source options on source step', async () => {
    const wrapper = await mountPage()
    await navigateToSourceStep(wrapper)
    const labels = ['Instagram', 'LinkedIn', 'Facebook', 'Discord', 'Od znajomego', 'W szkole', 'W internecie', 'Inne']
    for (const label of labels) {
      expect(wrapper.findAll('button').find(b => b.text().includes(label))).toBeTruthy()
    }
  })

  it('does not advance from source step without selecting a source', async () => {
    const wrapper = await mountPage()
    await navigateToSourceStep(wrapper)
    await clickButton(wrapper, 'Dalej')
    expect(wrapper.text()).toContain('Krok 3 z 4')
  })

  it('advances to consents step after selecting a source', async () => {
    const wrapper = await mountPage()
    await navigateToConsentsStep(wrapper)
    expect(wrapper.text()).toContain('Krok 4 z 4')
    expect(wrapper.text()).toContain('Ostatnie dwie rzeczy')
  })

  it('shows "Zapisz i wejdź" button on consents step', async () => {
    const wrapper = await mountPage()
    await navigateToConsentsStep(wrapper)
    expect(wrapper.findAll('button').find(b => b.text().includes('Zapisz i wejdź'))).toBeTruthy()
  })

  it('can navigate back from about step to welcome', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    await clickButton(wrapper, 'Wstecz')
    expect(wrapper.text()).toContain('Krok 1 z 4')
  })

  it('can navigate back from source step to about', async () => {
    const wrapper = await mountPage()
    await navigateToSourceStep(wrapper)
    await clickButton(wrapper, 'Wstecz')
    expect(wrapper.text()).toContain('Krok 2 z 4')
  })

  it('preserves form data when navigating back and forward', async () => {
    const wrapper = await mountPage()
    await navigateToAboutStep(wrapper)
    await wrapper.find('input[placeholder="Twoje imię"]').setValue('Anna')
    await wrapper.find('input[placeholder="np. Kraków"]').setValue('Gdańsk')
    await flushPromises()
    await clickButton(wrapper, 'Dalej')
    await clickButton(wrapper, 'Wstecz')
    expect(wrapper.find('input[placeholder="Twoje imię"]').element.value).toBe('Anna')
    expect(wrapper.find('input[placeholder="np. Kraków"]').element.value).toBe('Gdańsk')
  })
})
