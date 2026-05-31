<script setup lang="ts">
definePageMeta({
  layout: 'account',
})

useSeoMeta({
  title: 'Wydarzenia',
  description: 'Aktualne wydarzenie i rejestracja Hack4Krak.',
})

const { data: eventStatus } = await useApi('/event/status', {
  onResponseError: undefined,
})
const isEventLive = computed(() => eventStatus.value?.is_live ?? false)
</script>

<template>
  <div class="space-y-8">
    <header class="flex items-end justify-between gap-4 flex-wrap">
      <div>
        <h1 class="font-pixelify text-3xl lg:text-4xl leading-none">
          Wydarzenia
        </h1>
        <p class="text-sm text-muted mt-2">
          Aktualne wydarzenie i rejestracja.
        </p>
      </div>
    </header>

    <PanelEventEditionCardUpcoming :is-event-live="isEventLive" />

    <div class="border-2 border-surface-muted p-5">
      <h3 class="font-pixelify text-xl">
        Historia udziału
      </h3>
      <p class="text-sm text-muted mt-2">
        Wyniki i poprzednie edycje pojawią się tutaj po zakończeniu wydarzenia.
      </p>
    </div>
  </div>
</template>
