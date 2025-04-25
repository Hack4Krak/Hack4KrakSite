import { describe, expect, it } from 'vitest'
import { dayjs, humanizeDifference } from '~/utils/duration'

describe('utilsDuration', () => {
  describe('humanizeDifference', () => {
    it('formats days correctly', () => {
      expect(humanizeDifference(dayjs.duration(2, 'days').asMilliseconds())).toBe('2d 00:00:00')
      expect(humanizeDifference(dayjs.duration(1, 'day').asMilliseconds())).toBe('1d 00:00:00')
    })

    it('formats short durations correctly', () => {
      expect(humanizeDifference(dayjs.duration(5, 'hours').asMilliseconds())).toBe('05:00:00')
      expect(humanizeDifference(dayjs.duration(23, 'minutes').asMilliseconds())).toBe('00:23:00')
    })

    it('formats complex durations correctly', () => {
      expect(humanizeDifference(dayjs.duration({ days: 1, hours: 12, minutes: 30, seconds: 15 }).asMilliseconds()))
        .toBe('1d 12:30:15')
    })

    it('formats big durations correctly', () => {
      expect(humanizeDifference(dayjs.duration({ months: 5, days: 1, hours: 12, minutes: 30, seconds: 15 }).asMilliseconds()))
        .toBe('153d 12:30:15')
    })
  })
})
