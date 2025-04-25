import dayjs from 'dayjs'
import dayjsDuration from 'dayjs/plugin/duration'
import isBetween from 'dayjs/plugin/isBetween'

// Temporary solution for https://github.com/fumeapp/dayjs/issues/62
dayjs.extend(dayjsDuration)
dayjs.extend(isBetween)

function humanizeDifference(durationMs: number): string {
  const duration = dayjs.duration(durationMs)

  const totalDays = Math.floor(duration.asDays())
  const hours = String(duration.hours()).padStart(2, '0')
  const minutes = String(duration.minutes()).padStart(2, '0')
  const seconds = String(duration.seconds()).padStart(2, '0')

  const dayPart = totalDays > 0 ? `${totalDays}d ` : ''
  return `${dayPart}${hours}:${minutes}:${seconds}`
}

export { dayjs, humanizeDifference }
