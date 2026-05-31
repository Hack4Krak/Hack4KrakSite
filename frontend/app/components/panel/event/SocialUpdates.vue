<script setup lang="ts">
import { LANDING_SOCIALS } from '~~/content/landing/socials'

const socialLinks = LANDING_SOCIALS.filter(s => ['Discord', 'Instagram', 'LinkedIn'].includes(s.label))
</script>

<template>
  <PanelCard accent body class="relative overflow-hidden">
    <div class="relative z-10 grid grid-cols-1 gap-5 md:grid-cols-[minmax(0,1fr)_auto] md:items-center">
      <div>
        <PanelSectionTitle class="mb-3">
          Bądź na bieżąco
        </PanelSectionTitle>
        <h2 class="font-pixelify text-2xl leading-none">
          Dołącz do społeczności Hack4Krak
        </h2>
        <p class="mt-3 max-w-2xl text-sm leading-relaxed text-muted">
          Obserwuj aktualności, przypomnienia organizacyjne i komunikaty dla uczestników na naszych kanałach.
        </p>
      </div>

      <div class="flex flex-wrap gap-3 md:justify-end">
        <PanelActionButton
          v-for="social in socialLinks"
          :key="social.label"
          :to="social.to"
          :target="social.target"
          :icon="social.icon"
        >
          {{ social.label }}
        </PanelActionButton>
      </div>
    </div>
  </PanelCard>
</template>
