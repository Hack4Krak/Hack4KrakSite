type DateInput = string | Date

const FIRST_CHAR = /^./
const EVENT_TIMEZONE = 'Europe/Warsaw'
const TIME_FORMATTER = new Intl.DateTimeFormat('pl-PL', {
  hour: '2-digit',
  minute: '2-digit',
  timeZone: EVENT_TIMEZONE,
})
const POLISH_DAY_FORMATTER = new Intl.DateTimeFormat('pl-PL', {
  weekday: 'long',
  day: 'numeric',
  month: 'long',
  timeZone: EVENT_TIMEZONE,
})

export function toDate(value?: string | null) {
  return value ? new Date(value) : undefined
}

export function toTimestamp(value: DateInput) {
  return new Date(value).getTime()
}

export function formatTime(date: string) {
  return TIME_FORMATTER.format(new Date(date))
}

export function formatPolishDay(date: string) {
  return POLISH_DAY_FORMATTER.format(new Date(date)).replace(FIRST_CHAR, char => char.toUpperCase())
}

export { EVENT_TIMEZONE }
