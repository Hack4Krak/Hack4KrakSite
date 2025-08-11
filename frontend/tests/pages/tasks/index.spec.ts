import { mountSuspended } from '@nuxt/test-utils/runtime'
import { describe, expect, it, vi } from 'vitest'
import Map from '@/components/Map.vue'
import TasksPage from '@/pages/tasks/index.vue'

vi.mock('@/composables/useApi', () => ({
  useApi: vi.fn().mockResolvedValue({ data: { value: [] } }),
}))

vi.mock('@/composables/useAuth', () => ({
  useAuth: vi.fn().mockResolvedValue({ data: { value: 'skbidi' } }),
}))

describe('taskPage', () => {
  it('should handle authenticated error for completed tasks', async () => {
    const wrapper = await mountSuspended(TasksPage, {
      global: {
        components: {
          Map,
        },
      },
    })

    await wrapper.vm.$nextTick()

    expect(wrapper.findComponent(Map).exists()).toBe(true)
    expect(wrapper.findComponent(Map).props('elements')).toEqual([])
  })
})
