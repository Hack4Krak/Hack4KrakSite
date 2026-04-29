<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const event = LANDING_CONTENT.event

const { data: registrationInformation } = await useApi('/event/registration')

const registrationOpen = computed(() => registrationInformation.value?.is_open ?? false)
</script>

<template>
  <div class="flex flex-col items-center gap-3">
    <ElevatedButton
      :to="registrationOpen ? '/register' : '/panel'"
      :aria-label="registrationOpen ? 'Zapisz się na Hack4Krak CTF' : 'Otwórz panel'"
      class="scale-125"
    >
      {{ registrationOpen ? 'Zapisz się' : 'Start Gry' }}
    </ElevatedButton>
    <p v-if="registrationOpen" class="text-xs text-muted text-center">
      {{ event.dateDisplay }} · {{ event.venue.city }}
    </p>
  </div>
</template>
