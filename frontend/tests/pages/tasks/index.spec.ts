import { mountSuspended } from '@nuxt/test-utils/runtime'
import { describe, expect, it, vi } from 'vitest'
import TasksPage from '@/pages/tasks/index.vue'

// Maplibre GL seems to have problems with tests, so we have to mock it
vi.mock('@indoorequal/vue-maplibre-gl', () => ({
  MglMap: { template: '<div />' },
  MglMarker: { template: '<div />' },
}))

vi.mock('@/composables/useApi', () => ({
  useApi: vi.fn().mockResolvedValue({ data: { value: [] } }),
}))

vi.mock('@/composables/useAuth', () => ({
  useAuth: vi.fn().mockResolvedValue({ data: { value: 'skbidi' } }),
}))

describe('taskPage', () => {
  it('should handle authenticated error for completed tasks', async () => {
    const wrapper = await mountSuspended(TasksPage)

    await wrapper.vm.$nextTick()

    expect(wrapper.findComponent({ name: 'Map' }).exists()).toBe(true)
  })
})
