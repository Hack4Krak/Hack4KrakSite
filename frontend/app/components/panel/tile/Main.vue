<script setup>
import dayjs from 'dayjs'
import duration from 'dayjs/plugin/duration'
import isBetween from 'dayjs/plugin/isBetween'

const { data } = await useAuth('/event/info')

const { data: timeLeft } = useAsyncData('timeLeft', async () => calculateTimeLeft())
const eventMessage = computed(() => getEventState().message)

function getEventState() {
  // Temporary solution for https://github.com/fumeapp/dayjs/issues/62
  dayjs.extend(duration)
  dayjs.extend(isBetween)
  const now = dayjs()
  const eventStart = dayjs(data.value?.start_date)
  const eventEnd = dayjs(data.value?.end_date)

  if (now.isBetween(eventStart, eventEnd)) {
    return { message: 'Czas do zakończenia wydarzenia', diff: dayjs.duration(eventEnd.diff()) }
  } else if (now.isBefore(eventStart)) {
    return { message: 'Czas do rozpoczęcia wydarzenia', diff: dayjs.duration(eventStart.diff(now)) }
  } else {
    return { message: 'Wydarzenie jest zakończone od', diff: dayjs.duration(now.diff(eventEnd)) }
  }
}

function calculateTimeLeft() {
  return getEventState().diff.format('D[d] HH:mm:ss').replace(/^0*d /, '')
}

onMounted(() => {
  setInterval(() => {
    timeLeft.value = calculateTimeLeft()
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
