import type { ApiResponse } from '#open-fetch'
import type { Ref } from 'vue'
import { toDate } from '~/utils/date'

type EventInformation = ApiResponse<'info'>

type EventInformationDates = EventInformation | {
  start_date?: string | null
  end_date?: string | null
}

function getStageDate(eventInfo: EventInformation | undefined, stageType: 'event-start' | 'event-end') {
  return eventInfo?.stages.find(stage => stage.stage_type === stageType)?.start_date
}

export default async function useEventStartAndEnd(eventInformation?: Ref<EventInformationDates | null | undefined>) {
  const eventInfo = eventInformation ?? (await useApi('/event/info')).data

  const start = 'stages' in (eventInfo.value ?? {})
    ? toDate(getStageDate(eventInfo.value as EventInformation, 'event-start'))
    : toDate(eventInfo.value?.start_date)
  const end = 'stages' in (eventInfo.value ?? {})
    ? toDate(getStageDate(eventInfo.value as EventInformation, 'event-end'))
    : toDate(eventInfo.value?.end_date)

  return [start, end] as const
}
