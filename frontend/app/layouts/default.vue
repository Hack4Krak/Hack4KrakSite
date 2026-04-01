<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const { event } = LANDING_CONTENT
</script>

<template>
  <UBanner
    id="hack4krak-edition-3"
    icon="pixelarticons:flag"
    color="primary"
    :title="`Zapisy otwarte! Hack4Krak CTF — ${event.dateDisplay}, ${event.venue.city}`"
    :actions="[{
      label: 'Zapisz się',
      to: event.registrationUrl,
      color: 'neutral',
      variant: 'solid',
      size: 'xs',
      class: 'font-bold tracking-wide rounded-none border border-white/30 bg-black/30 hover:bg-black/50 text-white',
    }]"
    close
  />
  <Navbar />
  <UMain class="flex-1 flex flex-col" as="main">
    <slot />
  </UMain>
</template>
