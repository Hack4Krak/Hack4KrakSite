<script setup lang="ts">
import useEventStartAndEnd from '~/composables/useEventStartAndEnd'
import { dayjs, humanizeDifference } from '~/utils/duration'

const [eventStart, eventEnd] = await useEventStartAndEnd()
const { data: timeLeft, refresh: updateTimeLeft } = useAsyncData('timeLeft', async () => getEventState())

function getEventState() {
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

useRafFn(() => updateTimeLeft())
</script>

<template>
  <div
    class="panel-subcard justify-center items-center flex-col flex font-pixelify py-4 px-3 text-center"
    :style="{
      background: `linear-gradient(to right, ${timeLeft?.color} ${timeLeft?.percentage}%, transparent ${timeLeft?.percentage}%)`,
    }"
  >
    <span class="text-3xl lg:text-4xl tabular-nums">{{ timeLeft?.diff }}</span>
    <span v-if="!timeLeft?.hidePercentage" class="text-xs uppercase tracking-wider text-muted mt-1 tabular-nums">{{ timeLeft?.percentage }}%</span>
  </div>
</template>
