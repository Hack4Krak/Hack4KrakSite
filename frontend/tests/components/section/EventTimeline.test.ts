import { mountSuspended } from '@nuxt/test-utils/runtime'
import { describe, expect, it } from 'vitest'
import EventTimeline from '@/components/section/EventTimeline.vue'

const stages = [
  {
    name: 'Rejestracja uczestników',
    stage_type: 'informative' as const,
    start_date: '2026-05-23T08:00:00+02:00',
    end_date: '2026-05-23T09:30:00+02:00',
  },
  {
    name: 'Śniadanie',
    stage_type: 'informative' as const,
    start_date: '2026-05-23T09:00:00+02:00',
    end_date: '2026-05-23T11:00:00+02:00',
  },
  {
    name: 'Publikacja zadań',
    stage_type: 'event-start' as const,
    start_date: '2026-05-23T10:00:00+02:00',
    end_date: null,
  },
  {
    name: 'Obiad',
    stage_type: 'informative' as const,
    start_date: '2026-05-24T14:00:00+02:00',
    end_date: '2026-05-24T15:00:00+02:00',
  },
  {
    name: 'Koniec rozwiązywania zadań',
    stage_type: 'event-end' as const,
    start_date: '2026-05-24T16:00:00+02:00',
    end_date: null,
  },
]

describe('eventTimeline', () => {
  it('groups stages by day and renders time ranges', async () => {
    const wrapper = await mountSuspended(EventTimeline, {
      props: {
        stages,
        now: '2026-05-23T07:00:00+02:00',
      },
    })

    expect(wrapper.findAll('[data-timeline-day]')).toHaveLength(2)
    expect(wrapper.text()).toContain('08:00 - 09:30')
    expect(wrapper.text()).toContain('10:00')
    expect(wrapper.text()).toContain('14:00 - 15:00')
  })

  it('emphasizes non-informative stages', async () => {
    const wrapper = await mountSuspended(EventTimeline, {
      props: {
        stages,
        now: '2026-05-23T07:00:00+02:00',
      },
    })

    const keyStage = wrapper.find('[data-stage-type="event-start"] .text-sm')
    const informativeStage = wrapper.find('[data-stage-type="informative"] .text-sm')

    expect(keyStage.classes()).toContain('font-bold')
    expect(keyStage.classes()).toContain('text-primary')
    expect(informativeStage.classes()).not.toContain('font-bold')
  })

  it('marks the latest active stage as current and dims past stages', async () => {
    const wrapper = await mountSuspended(EventTimeline, {
      props: {
        stages,
        now: '2026-05-23T10:30:00+02:00',
      },
    })

    const currentStage = wrapper.find('[data-stage-type="event-start"]')
    const pastStage = wrapper.find('[data-stage-type="informative"][data-stage-state="past"]')
    const breakfastStage = wrapper.findAll('[data-stage-type="informative"]')[1]

    expect(currentStage.attributes('data-stage-state')).toBe('current')
    expect(currentStage.classes()).toContain('bg-primary/10')
    expect(pastStage.text()).toContain('Rejestracja uczestników')
    expect(pastStage.classes()).toContain('opacity-45')
    expect(breakfastStage.attributes('data-stage-state')).toBe('upcoming')
  })
})
