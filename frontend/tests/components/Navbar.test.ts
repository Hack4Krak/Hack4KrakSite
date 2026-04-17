import { mockNuxtImport, mountSuspended } from '@nuxt/test-utils/runtime'
import { describe, expect, it } from 'vitest'
import Navbar from '@/components/Navbar.vue'

const userData = ref<any>(null)

mockNuxtImport('useAuth', () => {
  return (..._args: any[]) => ({
    data: userData,
  })
})

describe('navbar', () => {
  it('displays username when user is authenticated', async () => {
    userData.value = { username: 'TestUser' }

    const wrapper = await mountSuspended(Navbar)
    expect(wrapper.text()).toContain('TestUser')
  })

  it('displays login link when user is not authenticated', async () => {
    userData.value = null

    const wrapper = await mountSuspended(Navbar)
    expect(wrapper.text()).toContain('Zaloguj się')
  })

  it('reactively updates navbar when auth data refreshes', async () => {
    userData.value = null

    const wrapper = await mountSuspended(Navbar)
    expect(wrapper.text()).toContain('Zaloguj się')
    expect(wrapper.text()).not.toContain('RefreshedUser')

    // Simulate refreshNuxtData() updating the useAuth data ref
    // (this is what happens after token refresh in openFetch plugin)
    userData.value = { username: 'RefreshedUser' }
    await nextTick()

    expect(wrapper.text()).toContain('RefreshedUser')
    expect(wrapper.text()).not.toContain('Zaloguj się')
  })
})
