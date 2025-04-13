import { mountSuspended } from '@nuxt/test-utils/runtime'

import { describe, expect, it } from 'vitest'
import Navbar from '~/components/Navbar.vue'

describe('navbar', () => {
  it('renders correctly', async () => {
    const wrapper = await mountSuspended(Navbar)
    expect(wrapper.text()).toContain('Hack4Krak')
  })

  it('toggles mobile menu on button click', async () => {
    const wrapper = await mountSuspended(Navbar)
    const button = wrapper.find('button[aria-label="Toogle navbar"]')
    expect(wrapper.find('[class*="h-screen"]').exists()).toBe(false)
    await button.trigger('click')
    expect(wrapper.find('[class*="h-screen"]').exists()).toBe(true)
  })

  it('closes mobile menu on route change', async () => {
    const wrapper = await mountSuspended(Navbar)
    const button = wrapper.find('button[aria-label="Toogle navbar"]')
    await button.trigger('click')
    expect(wrapper.find('[class*="h-screen"]').exists()).toBe(true)

    // Simulate route change
    const router = useRouter()
    await router.push('/leaderboard')
    await nextTick()

    expect(wrapper.find('[class*="h-screen"]').exists()).toBe(false)
  })
})
