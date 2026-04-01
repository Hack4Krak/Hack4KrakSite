<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'
</script>

<template>
  <section class="w-full py-12 lg:py-20 border-y-2 border-surface-muted">
    <div class="text-center mb-10 lg:mb-14">
      <p class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
        W liczbach
      </p>
      <h2 class="font-pixelify text-3xl lg:text-5xl text-default">
        Nasze dotychczasowe edycje
      </h2>
    </div>

    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 lg:gap-6">
      <div
        v-for="(stat, index) in LANDING_CONTENT.stats"
        :key="index"
        class="flex flex-col items-center justify-center p-6 lg:p-8 border-2 border-surface-muted
               hover:border-primary transition-colors duration-300 text-center gap-2"
      >
        <span class="font-pixelify text-4xl lg:text-6xl text-primary font-bold">
          {{ stat.value }}
        </span>
        <span class="text-sm lg:text-base text-muted uppercase tracking-widest">
          {{ stat.label }}
        </span>
      </div>
    </div>
  </section>
</template>
