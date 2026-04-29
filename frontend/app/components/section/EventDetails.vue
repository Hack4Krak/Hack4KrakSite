<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const { event } = LANDING_CONTENT

const { data: eventInformation } = await useApi('/event/info', {
  key: 'landing-event-info',
})
</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8 items-stretch">
    <SectionEventVenueMap :venue="event.venue" />

    <SectionEventTimeline :stages="eventInformation?.stages ?? []" />
  </div>

  <div class="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-6 lg:gap-8">
    <div class="hidden lg:block" />
    <p class="text-xs text-muted text-center lg:text-left">
      * Harmonogram może ulec drobnym zmianom — śledź nasze social media po aktualności.
    </p>
  </div>
</template>
