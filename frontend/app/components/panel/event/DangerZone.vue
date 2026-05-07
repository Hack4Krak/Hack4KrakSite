<script setup lang="ts">
defineProps<{
  hasTeam: boolean
  isLeader: boolean
  isRegistered: boolean
}>()

defineEmits<{
  teamAction: []
  cancelParticipation: []
}>()
</script>

<template>
  <article
    v-if="hasTeam || isRegistered"
    class="panel-card panel-card-body"
  >
    <p class="panel-section-title text-error mb-5">
      Strefa zagrożenia
    </p>

    <div class="grid grid-cols-1 gap-4">
      <div
        v-if="hasTeam"
        class="grid grid-cols-1 items-start gap-3 sm:grid-cols-[minmax(0,1fr)_auto]"
      >
        <div class="min-w-0">
          <p class="font-bold leading-tight">
            {{ isLeader ? 'Usuń drużynę' : 'Opuść drużynę' }}
          </p>
          <p class="mt-1 text-sm text-muted">
            {{ isLeader
              ? 'Drużyna zostanie usunięta wraz z zaproszeniami.'
              : 'Możesz wrócić z nowym zaproszeniem.' }}
          </p>
        </div>
        <button
          type="button"
          class="inline-flex w-fit cursor-pointer items-center border-2 border-error/70 px-3 py-1.5 text-xs font-bold uppercase tracking-wider text-error transition-colors hover:border-error hover:bg-error hover:text-default"
          @click="$emit('teamAction')"
        >
          {{ isLeader ? 'Usuń drużynę' : 'Opuść drużynę' }}
        </button>
      </div>

      <div
        v-if="isRegistered"
        class="grid grid-cols-1 items-start gap-3 border-t-2 border-surface-muted pt-4 sm:grid-cols-[minmax(0,1fr)_auto]"
      >
        <div class="min-w-0">
          <p class="font-bold leading-tight">
            Wypisz się z wydarzenia
          </p>
          <p class="mt-1 text-sm text-muted">
            {{ hasTeam
              ? `Najpierw ${isLeader ? 'usuń drużynę' : 'opuść drużynę'}, potem możesz wypisać się z wydarzenia.`
              : 'Usunie Twoje zgłoszenie i zablokuje kod QR do czasu ponownej rejestracji.' }}
          </p>
        </div>
        <button
          type="button"
          :disabled="hasTeam"
          class="inline-flex w-fit items-center border-2 border-error/70 px-3 py-1.5 text-xs font-bold uppercase tracking-wider text-error transition-colors enabled:cursor-pointer enabled:hover:border-error enabled:hover:bg-error enabled:hover:text-default disabled:cursor-not-allowed disabled:border-surface-muted disabled:text-muted"
          :title="hasTeam ? 'Najpierw opuść lub usuń drużynę' : undefined"
          @click="!hasTeam && $emit('cancelParticipation')"
        >
          {{ hasTeam ? 'Najpierw drużyna' : 'Wypisz się' }}
        </button>
      </div>
    </div>
  </article>
</template>
