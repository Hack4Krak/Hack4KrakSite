<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const event = LANDING_CONTENT.event

const { data: registrationInformation } = await useApi('/event/registration')

const registrationOpen = useRegistrationOpen(registrationInformation)
</script>

<template>
  <UBanner
    v-if="registrationOpen"
    id="hack4krak-edition-3"
    icon="pixelarticons:flag"
    color="primary"
    :title="`Zapisy otwarte! Hack4Krak CTF dla uczniów szkół ponadpodstawowych - ${event.dateDisplay}, ${event.venue.city}`"
    close
  />
  <Navbar />
  <UMain class="flex-1 flex flex-col" as="main">
    <slot />
  </UMain>
</template>
