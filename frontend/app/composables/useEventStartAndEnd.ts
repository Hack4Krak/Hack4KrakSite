import type { Ref } from 'vue'
import { toDate } from '~/utils/date'

interface EventStageInformation {
  stage_type: 'event-start' | 'event-end' | 'informative'
  start_date: string
  end_date?: string | null
}

interface EventInformationDates {
  start_date?: string | null
  end_date?: string | null
  stages?: EventStageInformation[]
}

function getStageDate(eventInfo: EventInformationDates | undefined, stageType: 'event-start' | 'event-end') {
  const stage = eventInfo?.stages?.find(item => item.stage_type === stageType)
  return stageType === 'event-end'
    ? stage?.end_date ?? stage?.start_date
    : stage?.start_date
}

export default async function useEventStartAndEnd(eventInformation?: Ref<EventInformationDates | null | undefined>) {
  const eventInfo = eventInformation ?? (await useApi('/event/info')).data

  const start = 'stages' in (eventInfo.value ?? {})
    ? toDate(getStageDate(eventInfo.value ?? undefined, 'event-start'))
    : toDate(eventInfo.value?.start_date)
  const end = 'stages' in (eventInfo.value ?? {})
    ? toDate(getStageDate(eventInfo.value ?? undefined, 'event-end'))
    : toDate(eventInfo.value?.end_date)

  return [start, end] as const
}
