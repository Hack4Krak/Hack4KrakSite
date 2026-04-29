<script setup lang="ts">
import {
  LANDING_FORMAT_CARD,
  LANDING_HIGHLIGHTS,
} from '~~/content/landing/highlights'
</script>

<template>
  <div class="grid gap-4 lg:grid-cols-3 lg:gap-6">
    <div
      v-for="highlight in LANDING_HIGHLIGHTS"
      :key="highlight.title"
      class="relative flex min-h-76 flex-col overflow-hidden border-2 bg-default p-6 lg:p-7"
      :style="{ borderColor: highlight.color }"
    >
      <div class="flex items-start justify-between gap-4">
        <div>
          <span class="block font-pixelify text-5xl font-bold leading-none lg:text-7xl" :style="{ color: highlight.color }">
            {{ highlight.accent }}
          </span>
        </div>

        <NuxtImg
          :src="highlight.image"
          :alt="highlight.imageAlt"
          class="pointer-events-none h-24 w-auto shrink-0 rendering-pixelated lg:h-28"
        />
      </div>

      <div class="mt-6">
        <h3 class="max-w-[16rem] text-xl font-bold text-default lg:text-2xl">
          {{ highlight.title }}
        </h3>
        <p class="mt-3 max-w-[22rem] text-sm leading-relaxed text-default/72">
          {{ highlight.description }}
        </p>
      </div>
    </div>
  </div>

  <div class="mt-4 grid gap-4 lg:mt-6 lg:grid-cols-3 lg:gap-6">
    <div
      class="relative flex min-h-76 flex-col overflow-hidden border-2 bg-default p-6 lg:col-span-2 lg:p-7"
      :style="{ borderColor: LANDING_FORMAT_CARD.color }"
    >
      <div class="flex h-full items-center justify-between gap-8">
        <div class="py-4 lg:py-6">
          <div class="flex items-end gap-3">
            <span class="font-pixelify text-4xl font-bold leading-none lg:text-5xl" :style="{ color: LANDING_FORMAT_CARD.color }">
              {{ LANDING_FORMAT_CARD.accent }}
            </span>
            <h3 class="text-xl font-bold text-default lg:text-2xl lg:whitespace-nowrap">
              {{ LANDING_FORMAT_CARD.title }}
            </h3>
          </div>
          <p class="mt-5 max-w-[31rem] text-sm leading-relaxed text-default/72 lg:text-base">
            {{ LANDING_FORMAT_CARD.description }}
          </p>
        </div>

        <NuxtImg
          :src="LANDING_FORMAT_CARD.image"
          :alt="LANDING_FORMAT_CARD.imageAlt"
          class="pointer-events-none hidden self-end h-52 w-auto shrink-0 rendering-pixelated lg:block"
        />
      </div>
    </div>

    <div class="flex min-h-76 flex-col items-center justify-center gap-8 border-2 border-primary bg-default p-6 lg:p-7 text-center">
      <p class="text-xl lg:text-2xl font-bold ">
        Masz więcej pytań dotyczących zawodów?
      </p>
      <ElevatedButton to="/docs/faq" class="lg:text-xl bg-primary!">
        Odwiedź stronę FAQ
      </ElevatedButton>
    </div>
  </div>
</template>
