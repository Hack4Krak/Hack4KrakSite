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
  <PanelCard
    v-if="hasTeam || isRegistered"
    body
  >
    <PanelSectionTitle class="mb-5 text-error">
      Strefa zagrożenia
    </PanelSectionTitle>

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
        <PanelActionButton
          tone="danger"
          @click="$emit('teamAction')"
        >
          {{ isLeader ? 'Usuń drużynę' : 'Opuść drużynę' }}
        </PanelActionButton>
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
        <PanelActionButton
          :disabled="hasTeam"
          tone="danger"
          :title="hasTeam ? 'Najpierw opuść lub usuń drużynę' : undefined"
          @click="!hasTeam && $emit('cancelParticipation')"
        >
          {{ hasTeam ? 'Najpierw opuść drużynę' : 'Wypisz się' }}
        </PanelActionButton>
      </div>
    </div>
  </PanelCard>
</template>
