import type { Ref } from 'vue'
import { toDate } from '~/utils/date'

interface EventInformationDates {
  start_date?: string | null
  end_date?: string | null
}

export default async function useEventStartAndEnd(eventInformation?: Ref<EventInformationDates | null | undefined>) {
  const eventInfo = eventInformation ?? (await useApi('/event/info')).data

  const start = toDate(eventInfo.value?.start_date)
  const end = toDate(eventInfo.value?.end_date)

  return [start, end] as const
}
