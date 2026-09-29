<script setup lang="ts">
import { ORGANIZER_URL, PARTNER_MAIL_URL } from '~~/content/landing/community'
import LANDING_CONTENT from '~~/content/landing/page'

const { event } = LANDING_CONTENT
const { proxy } = useScriptUmamiAnalytics()
</script>

<template>
  <section class="flex flex-col border-t-2 border-surface-muted">
    <UContainer class="relative z-10 flex flex-col items-center gap-8 pt-24 text-center lg:pt-32">
      <h2 class="font-pixelify text-3xl text-balance text-default lg:text-5xl">
        Do startu {{ event.edition }}. edycji zostało
      </h2>

      <LandingCountdown size="xl" />

      <div class="mt-4 flex flex-wrap items-center justify-center gap-6">
        <ElevatedButton
          :to="PARTNER_MAIL_URL"
          background="var(--ui-primary)"
          class="text-base lg:text-lg"
          @click="proxy.track('partner_cta_click', { location: 'finale' })"
        >
          Zostań sponsorem
        </ElevatedButton>
        <ElevatedButton :to="ORGANIZER_URL" target="_blank" variant="light" background="var(--ui-primary)" class="text-base lg:text-lg">
          Zostań organizatorem
        </ElevatedButton>
      </div>
    </UContainer>

    <!-- Kraków at night as the base of the page: a short fade into the sky, then the skyline -->
    <div class="relative -mt-6 h-40 bg-gradient-to-b from-transparent to-[#000b25] lg:h-48" aria-hidden="true">
      <div class="absolute inset-x-0 bottom-0 h-[135px] bg-[url(/img/sprites/skyline-night.png)] bg-[length:1200px_135px] bg-bottom bg-repeat-x rendering-pixelated" />
    </div>
  </section>
</template>
