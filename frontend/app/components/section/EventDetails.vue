<script setup lang="ts">
const { data: eventInformation, status } = await useApi('/event/info')
</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8 items-stretch">
    <SectionEventVenueMap />

    <SectionEventTimeline :stages="eventInformation?.stages ?? []" />
  </div>

  <div v-if="status === 'success'" class="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8">
    <div class="hidden lg:block" />
    <p class="text-xs text-muted text-center lg:text-left">
      * Godziny podane są w lokalnym czasie wydarzenia (Europe/Warsaw). <br> Harmonogram może ulec drobnym zmianom — śledź nasze social media po aktualności.
    </p>
  </div>
</template>
