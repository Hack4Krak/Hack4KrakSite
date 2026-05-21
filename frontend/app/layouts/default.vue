<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const event = LANDING_CONTENT.event
</script>

<template>
  <UBanner
    id="hack4krak-location-updated"
    icon="pixelarticons:map-pin"
    color="primary"
    :title="`Uwaga! Zmieniono miejsce wydarzenia na ${event.venue.name}`"
    close
  />
  <Navbar />
  <UMain class="flex-1 flex flex-col" as="main">
    <slot />
  </UMain>
</template>
