<script setup lang="ts">
import { dayjs, humanizeDifference } from '~/utils/duration'

const { data } = await useAuth('/event/info')

const { data: timeLeft, refresh: updateTimeLeft } = useAsyncData('timeLeft', async () => getEventState())

function getEventState() {
  const now = dayjs()
  const eventStart = dayjs(data.value?.start_date)
  const eventEnd = dayjs(data.value?.end_date)

  if (now.isBetween(eventStart, eventEnd)) {
    const totalDuration = eventEnd.diff(eventStart)
    const elapsed = now.diff(eventStart)
    const percentage = Math.round((elapsed / totalDuration) * 10000) / 100
    return { diff: humanizeDifference(elapsed), percentage, color: 'rgba(110, 235, 131, 0.1)' }
  } else if (now.isBefore(eventStart)) {
    return { diff: humanizeDifference(eventStart.diff(now)), percentage: 100, hidePercentage: true, color: 'rgba(246, 178, 22, 0.1)' }
  } else {
    return { diff: humanizeDifference(now.diff(eventEnd)), percentage: 100, color: 'rgba(246, 178, 22, 0.1)' }
  }
}

useRafFn(() => updateTimeLeft())
</script>

<template>
  <div
    class="col-span-2 justify-center items-center flex-col flex font-pixelify py-3"
    :style="{
      background: `linear-gradient(to right, ${timeLeft?.color} ${timeLeft?.percentage}%, transparent ${timeLeft?.percentage}%)`,
    }"
  >
    <span class="text-5xl">{{ timeLeft?.diff }}</span>
    <span v-if="!timeLeft?.hidePercentage">{{ timeLeft?.percentage }}%</span>
  </div>
</template>
