<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const event = LANDING_CONTENT.event
</script>

<template>
  <div class="w-full border-2 border-primary overflow-hidden">
    <div class="grid grid-cols-1 lg:grid-cols-[1fr_auto_1fr] gap-0 divide-y-2 lg:divide-y-0 lg:divide-x-2 divide-surface-muted">
      <div class="flex flex-col justify-center gap-4 px-6 py-6 lg:px-8 lg:py-8">
        <p class="text-[10px] font-bold tracking-[0.3em] uppercase text-muted">
          Następna edycja
        </p>

        <div class="flex items-center gap-3">
          <UIcon name="pixelarticons:calendar" class="text-primary flex-shrink-0 size-5" />
          <span class="font-pixelify text-xl lg:text-2xl text-primary font-bold leading-tight whitespace-nowrap">
            {{ event.dateDisplay }}
          </span>
        </div>

        <div class="flex items-start gap-3">
          <UIcon name="pixelarticons:map-pin" class="text-primary flex-shrink-0 size-5 mt-0.5" />
          <div class="text-sm leading-snug">
            <p class="text-default font-semibold">
              {{ event.venue.name }}
            </p>
            <p class="text-muted">
              {{ event.venue.address }}
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-center px-6 py-6 lg:px-12 lg:py-8 bg-primary/5">
        <EventCountdown :show-fallback-date="false" />
      </div>

      <div class="flex flex-col items-center justify-center gap-3 px-6 py-6 lg:px-8 lg:py-8">
        <ElevatedButton to="/register" class="text-base">
          Zapisz się teraz
        </ElevatedButton>
        <div class="flex items-center gap-4 text-xs text-muted">
          <span class="flex items-center gap-1.5">
            <span class="size-1.5 bg-primary shrink-0" />
            Bezpłatny udział
          </span>
          <span class="flex items-center gap-1.5">
            <span class="size-1.5 bg-primary flex-shrink-0" />
            Szkoły średnie
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
