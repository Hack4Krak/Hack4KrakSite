<script setup lang="ts">
import { LANDING_SOCIALS } from '~~/content/landing/socials'

const socialLinks = LANDING_SOCIALS.filter(s => ['Discord', 'Instagram', 'LinkedIn'].includes(s.label))
</script>

<template>
  <article class="panel-card panel-card-accent panel-card-body overflow-hidden relative">
    <div class="relative z-10 grid grid-cols-1 gap-5 md:grid-cols-[minmax(0,1fr)_auto] md:items-center">
      <div>
        <p class="panel-section-title mb-3">
          Bądź na bieżąco
        </p>
        <h2 class="font-pixelify text-2xl leading-none">
          Dołącz do społeczności Hack4Krak
        </h2>
        <p class="panel-copy">
          Obserwuj aktualności, przypomnienia organizacyjne i komunikaty dla uczestników na naszych kanałach.
        </p>
      </div>

      <div class="flex flex-wrap gap-3 md:justify-end">
        <NuxtLink
          v-for="social in socialLinks"
          :key="social.label"
          :to="social.to"
          :target="social.target"
          class="inline-flex items-center gap-2 border-2 border-primary/40 bg-default/70 px-4 py-2 text-sm font-bold transition-colors hover:border-primary hover:bg-primary hover:text-default"
        >
          <UIcon :name="social.icon" class="size-5" />
          {{ social.label }}
        </NuxtLink>
      </div>
    </div>
  </article>
</template>
