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
        Możesz zarejestrować się solo i dobrać drużynę później. Wpadnij na Discorda Hack4Krak, poznaj innych uczestników albo załóż własny skład.
      </p>
    </header>

    <PanelTeamManagementWorkspace
      v-model:registration-team="team"
      redirect-to="/panel/event/register"
      :show-danger-zone="false"
    />

    <p v-if="team.confirmed" class="text-xs text-muted flex items-center gap-2">
      <span class="size-2 bg-success shrink-0" />
      Drużyna gotowa. Możesz przejść do następnego kroku.
    </p>
    <p v-else class="text-xs text-muted gap-2">
      <span class="size-2 bg-primary shrink-0" />
      Nie masz jeszcze drużyny? <b>Ale to jest jakby okej!</b> Możesz przejść dalej solo, ale w dniu wydarzenia musisz już mieć drużynę, którą znajdziesz na naszym Discordzie.
    </p>
  </section>
</template>
