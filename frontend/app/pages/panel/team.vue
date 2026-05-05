<script setup lang="ts">
definePageMeta({
  middleware: ['event-registration-guard'],
})

useSeoMeta({
  title: 'Panel drużyny',
  description: 'Zarządzaj swoją drużyną, zapraszaj nowych członków i sprawdzaj postępy!',
})
</script>

<template>
  <div class="panel-page">
    <UContainer class="w-full py-8 lg:py-10 space-y-8">
      <header class="panel-hero">
        <NuxtLink
          to="/panel/event"
          class="text-xs uppercase tracking-wider text-muted hover:text-default flex items-center gap-1.5 mb-4 transition-colors"
        >
          <UIcon name="pixelarticons:arrow-left" class="size-4" />
          Wróć do panelu wydarzenia
        </NuxtLink>
        <p class="panel-hero-kicker">
          Hack4Krak 2026
        </p>
        <h1 class="panel-title">
          Twoja drużyna
        </h1>
        <p class="panel-copy">
          W jednym miejscu utworzysz drużynę, dołączysz do istniejącej i zarządzisz składem.
        </p>
      </header>

      <PanelTeamManagementWorkspace redirect-to="/panel/team" />
    </UContainer>
  </div>
</template>
