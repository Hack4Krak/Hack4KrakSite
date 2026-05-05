<script setup lang="ts">
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const team = defineModel<EventRegistrationDraft['team']>('team', { required: true })
</script>

<template>
  <section class="space-y-6">
    <header>
      <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
        Twoja <span class="text-primary">drużyna.</span>
      </h2>
      <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
        Każdy udział jest przypisany do drużyny. Jeśli grasz solo, po prostu utwórz drużynę jednoosobową i przejdź dalej.
      </p>
    </header>

    <PanelTeamManagementWorkspace
      v-model:registration-team="team"
      redirect-to="/panel/event/register"
      :allow-external-join="false"
    />

    <p v-if="team.confirmed" class="text-xs text-muted flex items-center gap-2">
      <span class="size-2 bg-success shrink-0" />
      Drużyna gotowa. Możesz przejść do następnego kroku.
    </p>
  </section>
</template>
