<script setup lang="ts">
definePageMeta({
  middleware: [
    'event-access-guard',
  ],
})

useSeoMeta({
  title: 'Ranking',
  description: 'Zobacz aktualny ranking drużyn i ich punkty w czasie rzeczywistym!',
})
</script>

<template>
  <div class="my-5 text-center flex flex-col gap-5">
    <h1 class="font-bold text-5xl">
      Punktacja
    </h1>
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
