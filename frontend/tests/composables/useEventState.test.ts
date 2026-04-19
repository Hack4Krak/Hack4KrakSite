import { beforeEach, describe, expect, it, vi } from 'vitest'
import useEventState from '~/composables/useEventState'
import { dayjs } from '~/utils/duration'

vi.mock('~/composables/useEventStartAndEnd', () => ({
  default: vi.fn(),
}))

describe('useEventState', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe.each([
    {
      name: 'before event',
      setup: () => {
        const now = dayjs()
        const eventStart = now.add(1, 'hour')
        const eventEnd = now.add(2, 'hours')
        return [eventStart.toDate(), eventEnd.toDate()]
      },
      expectations: {
        isNotNull: true,
        percentage: 100,
        hasHidePercentage: true,
        color: 'rgba(246, 178, 22, 0.1)',
        properties: ['color', 'diff', 'hidePercentage', 'percentage'],
      },
    },
    {
      name: 'during event (50%)',
      setup: () => {
        const now = dayjs()
        const eventStart = now.subtract(1, 'hour')
        const eventEnd = now.add(1, 'hour')
        return [eventStart.toDate(), eventEnd.toDate()]
      },
      expectations: {
        isNotNull: true,
        percentage: 50,
        hasHidePercentage: false,
        color: 'rgba(110, 235, 131, 0.1)',
        properties: ['color', 'diff', 'percentage'],
      },
    },
    {
      name: 'during event (75%)',
      setup: () => {
        const now = dayjs()
        const eventStart = now.subtract(3, 'hour')
        const eventEnd = now.add(1, 'hour')
        return [eventStart.toDate(), eventEnd.toDate()]
      },
      expectations: {
        isNotNull: true,
        percentage: 75,
        hasHidePercentage: false,
        color: 'rgba(110, 235, 131, 0.1)',
        properties: ['color', 'diff', 'percentage'],
      },
    },
    {
      name: 'after event',
      setup: () => {
        const now = dayjs()
        const eventStart = now.subtract(2, 'hours')
        const eventEnd = now.subtract(1, 'hour')
        return [eventStart.toDate(), eventEnd.toDate()]
      },
      expectations: {
        isNotNull: true,
        percentage: 100,
        hasHidePercentage: false,
        color: 'rgba(246, 178, 22, 0.1)',
        properties: ['color', 'diff', 'percentage'],
      },
    },
  ])('$name', ({ setup, expectations }) => {
    it('returns correct state', async () => {
      const { default: mockUseEventStartAndEnd } = await import('~/composables/useEventStartAndEnd')
      mockUseEventStartAndEnd.mockResolvedValueOnce(setup())

      const result = await useEventState()

      expect(Object.keys(result).sort()).toEqual(expectations.properties)
    })
  })
})
