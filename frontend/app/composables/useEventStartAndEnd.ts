export default async function useEventStartAndEnd() {
  const { data: eventInformation } = await useApi('/event/info')

  const start = eventInformation.value?.start_date ? new Date(eventInformation.value.start_date) : undefined
  const end = eventInformation.value?.end_date ? new Date(eventInformation.value.end_date) : undefined

  return [start, end] as const
}
