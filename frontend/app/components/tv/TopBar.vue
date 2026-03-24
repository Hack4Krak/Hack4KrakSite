<script setup lang="ts">
import dayjs from 'dayjs'

const props = defineProps<{
  startDate?: string
  endDate?: string
  lunchStart?: string
  lunchEnd?: string
}>()

const now = ref(new Date())
useIntervalFn(() => {
  now.value = new Date()
}, 1000)

const isLunchTime = computed(() => {
  if (!props.lunchStart || !props.lunchEnd)
    return false
  const current = dayjs(now.value)
  const startParts = props.lunchStart.split(':').map(Number)
  const endParts = props.lunchEnd.split(':').map(Number)

  const lunchStartTime = current.hour(startParts[0] ?? 0).minute(startParts[1] ?? 0).second(0)
  const lunchEndTime = current.hour(endParts[0] ?? 0).minute(endParts[1] ?? 0).second(0)

  return current.isAfter(lunchStartTime) && current.isBefore(lunchEndTime)
})

const timeLeft = computed(() => {
  if (!props.endDate)
    return null
  const diff = Math.max(0, new Date(props.endDate).getTime() - now.value.getTime())
  return {
    hours: Math.floor(diff / (1000 * 60 * 60)),
    minutes: Math.floor(diff / (1000 * 60)) % 60,
    seconds: Math.floor(diff / 1000) % 60,
    total: diff,
  }
})

const currentTimeFormatted = computed(() =>
  dayjs(now.value).format('HH:mm:ss'),
)

function pad(n: number): string {
  return String(n).padStart(2, '0')
}
</script>

<template>
  <div class="flex items-center justify-between px-8 py-3 bg-surface-muted border-b border-surface-muted">
    <div class="flex items-center gap-3">
      <UIcon name="pixelarticons:calendar" class="text-2xl text-primary" />
      <span class="font-pixelify text-2xl text-primary tracking-wider">Hack4Krak</span>
    </div>

    <div class="flex items-center gap-6">
      <template v-if="isLunchTime">
        <div class="flex items-center gap-3 bg-primary/15 px-6 py-2 border border-primary/40">
          <UIcon name="pixelarticons:coin" class="text-3xl text-primary animate-pulse" />
          <span class="text-2xl font-bold text-primary font-pixelify tracking-wide">PORA OBIADOWA</span>
          <UIcon name="pixelarticons:coin" class="text-3xl text-primary animate-pulse" />
        </div>
      </template>
      <template v-else>
        <div v-if="timeLeft && timeLeft.total > 0" class="flex items-center gap-3">
          <UIcon name="pixelarticons:clock" class="text-2xl text-text-muted" />
          <span class="text-sm text-text-muted uppercase tracking-wider">Do końca</span>
          <span class="text-2xl font-bold font-roboto tabular-nums text-text-default">
            {{ pad(timeLeft.hours) }}:{{ pad(timeLeft.minutes) }}:{{ pad(timeLeft.seconds) }}
          </span>
        </div>
        <div v-else-if="timeLeft && timeLeft.total === 0" class="flex items-center gap-3">
          <span class="text-2xl font-bold text-red-450 font-pixelify">KONIEC WYDARZENIA</span>
        </div>
      </template>
    </div>

    <div class="flex items-center gap-3">
      <UIcon name="pixelarticons:clock" class="text-2xl text-text-muted" />
      <span class="text-3xl font-bold font-roboto tabular-nums text-text-default">
        {{ currentTimeFormatted }}
      </span>
    </div>
  </div>
</template>
