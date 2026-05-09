<script setup lang="ts">
definePageMeta({
  layout: 'panel',
  middleware: ['event-registration-guard'],
})

useSeoMeta({
  title: 'Panel drużyny',
  description: 'Zarządzaj swoją drużyną, zapraszaj nowych członków i sprawdzaj postępy!',
})
</script>

<template>
  <UContainer class="w-full py-8 lg:py-10 space-y-8">
    <PanelHero
      kicker="Hack4Krak 2026"
      title="Twoja drużyna"
      description="W jednym miejscu utworzysz drużynę, dołączysz do istniejącej i zarządzisz składem."
      back-to="/panel/event"
      back-label="Wróć do panelu wydarzenia"
    />

    <PanelTeamManagementWorkspace redirect-to="/panel/team" />
  </UContainer>
</template>
