import { dayjs, humanizeDifference } from '~/utils/duration'

export default async function useEventState() {
  const [eventStart, eventEnd] = await useEventStartAndEnd()

  const now = dayjs()

  if (!eventStart || !eventEnd) {
    return null
  }

  if (now.isBetween(eventStart, eventEnd)) {
    const totalDuration = dayjs(eventEnd).diff(eventStart)
    const elapsed = now.diff(eventStart)
    const percentage = Math.round((elapsed / totalDuration) * 10000) / 100
    return { diff: humanizeDifference(elapsed), percentage, color: 'rgba(110, 235, 131, 0.1)' }
  } else if (now.isBefore(eventStart)) {
    return { diff: humanizeDifference(dayjs(eventStart).diff(now)), percentage: 100, hidePercentage: true, color: 'rgba(246, 178, 22, 0.1)' }
  } else {
    return { diff: humanizeDifference(now.diff(eventEnd)), percentage: 100, color: 'rgba(246, 178, 22, 0.1)' }
  }
}
