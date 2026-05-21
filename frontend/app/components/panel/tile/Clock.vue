<script setup lang="ts">
import useEventStartAndEnd from '~/composables/useEventStartAndEnd'
import { dayjs, humanizeDifference } from '~/utils/duration'

const [eventStart, eventEnd] = await useEventStartAndEnd()
const now = useNow({ interval: 1000 })

const display = computed(() => {
  if (!eventStart || !eventEnd)
    return null
  const n = dayjs(now.value)
  if (n.isBefore(eventStart))
    return { icon: 'pixelarticons:hourglass', label: 'Do startu', value: humanizeDifference(dayjs(eventStart).diff(n)) }
  if (n.isBefore(eventEnd))
    return { icon: 'pixelarticons:clock', label: 'Do końca', value: humanizeDifference(dayjs(eventEnd).diff(n)) }
  return { icon: 'pixelarticons:flag', label: 'Event zakończony', value: null }
})
</script>

<template>
  <span v-if="display" class="inline-flex items-center gap-1.5">
    <UIcon :name="display.icon" class="size-4 text-primary" />
    <span>{{ display.label }}:</span>
    <span v-if="display.value" class="font-pixelify tabular-nums text-primary">{{ display.value }}</span>
  </span>
</template>
