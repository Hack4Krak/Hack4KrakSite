export default function useEventStartAndEnd() {
  const { data: eventInformation } = useLazyApi('/event/info')

  const stages = eventInformation.value?.stages
  const eventStartStage = stages?.find(s => s.stage_type === 'event-start')
  const eventEndStage = stages?.find(s => s.stage_type === 'event-end')
  const start = eventStartStage?.start_date ? new Date(eventStartStage.start_date) : undefined
  const end = eventEndStage?.start_date ? new Date(eventEndStage.start_date) : undefined

  return [start, end]
}
