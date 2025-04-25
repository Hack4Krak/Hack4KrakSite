<script setup lang="ts">
import { dayjs, humanizeDifference } from '~/utils/duration'

const { data } = await useAuth('/event/info')

const { data: timeLeft } = useAsyncData('timeLeft', async () => getEventState().diff)
const eventMessage = computed(() => getEventState().message)

function getEventState() {
  const now = dayjs()
  const eventStart = dayjs(data.value?.start_date)
  const eventEnd = dayjs(data.value?.end_date)

  if (now.isBetween(eventStart, eventEnd)) {
    return { message: 'Czas do zakończenia wydarzenia', diff: humanizeDifference(eventEnd.diff()) }
  } else if (now.isBefore(eventStart)) {
    return { message: 'Czas do rozpoczęcia wydarzenia', diff: humanizeDifference(eventStart.diff(now)) }
  } else {
    return { message: 'Wydarzenie jest zakończone od', diff: humanizeDifference(now.diff(eventEnd)) }
  }
}

onMounted(() => {
  setInterval(() => {
    timeLeft.value = getEventState().diff
  }, 1000)
})
</script>

<template>
  <div class="flex flex-col m-5 md:m-10 gap-5">
    <h2 class="text-2xl font-bold">
      {{ eventMessage }}
    </h2>
    <span class="w-full text-center text-6xl font-bold">{{ timeLeft }}</span>
    <h2 class="text-2xl font-bold mt-5">
      Wyślij flagę
    </h2>
    <PanelFlagForm />
  </div>
</template>
