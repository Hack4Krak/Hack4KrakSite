<script setup lang="ts">
import dayjs from 'dayjs'

definePageMeta({
  middleware: [
    'event-access-guard',
  ],
})

useSeoMeta({
  title: 'Ranking',
  description: 'Zobacz aktualny ranking drużyn i ich punkty w czasie rzeczywistym!',
})

const { data: eventInformation } = await useLazyApi('/event/info')

const isLeaderboardFrozen = computed(() => {
  const endDate = eventInformation.value?.end_date

  if (!endDate)
    return false

  const freezeStart = dayjs(endDate).subtract(1, 'hour')
  const now = dayjs()
  return now.isAfter(freezeStart) && now.isBefore(dayjs(endDate))
})
</script>

<template>
  <div class="my-5 text-center flex flex-col gap-5">
    <h1 class="font-bold text-5xl">
      Punktacja
    </h1>
    <UAlert
      v-if="isLeaderboardFrozen"
      icon="pixelarticons:alert"
      title="Ranking jest zamrożony"
      description="W ostatniej godzinie CTF wyniki są celowo ukryte. Aktualny ranking zostanie ujawniony po zakończeniu wydarzenia"
      color="warning"
      variant="subtle"
      class="mx-auto w-full max-w-4xl text-left"
    />
    <div class="overflow-x-auto h-[80vh]">
      <div class="h-full min-w-200 px-10">
        <LeaderboardChart />
      </div>
    </div>
    <div class="overflow-x-auto">
      <LeaderboardTable />
    </div>
  </div>
</template>
