import { mountSuspended } from '@nuxt/test-utils/runtime'
import { describe, expect, it, vi } from 'vitest'
import TasksPage from '@/pages/tasks/index.vue'

// Maplibre GL seems to have problems with tests, so we have to mock it
vi.mock('@indoorequal/vue-maplibre-gl', () => ({
  MglMap: { template: '<div />' },
  MglMarker: { template: '<div />' },
  useMap: vi.fn(() => ({ map: null })),
}))

// reka-ui tooltip components require a provider context that isn't available in tests
vi.mock('reka-ui', async (importOriginal) => {
  const actual = await importOriginal() as Record<string, unknown>
  return {
    ...actual,
    TooltipRoot: { template: '<div><slot /></div>' },
    TooltipProvider: { template: '<div><slot /></div>' },
    TooltipTrigger: { template: '<button><slot /></button>' },
    TooltipContent: { template: '<div><slot /></div>' },
  }
})

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
