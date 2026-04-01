<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const { event } = LANDING_CONTENT
</script>

<template>
  <div class="flex flex-col items-center gap-3">
    <NuxtLink :to="event.registrationOpen ? event.registrationUrl : '/panel'" :aria-label="event.registrationOpen ? 'Zapisz się na Hack4Krak CTF' : 'Otwórz panel'">
      <ElevatedButton class="scale-125">
        {{ event.registrationOpen ? 'Zapisz się' : 'Start Gry' }}
      </ElevatedButton>
    </NuxtLink>
    <p v-if="event.registrationOpen" class="text-xs text-muted text-center">
      {{ event.dateDisplay }} · {{ event.venue.city }}
    </p>
  </div>
</template>
