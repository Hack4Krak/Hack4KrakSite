<script setup lang="ts">
const createTeamModal = ref(false)
const invitationsModal = ref(false)
</script>

<template>
  <PanelModalCreateTeam v-model="createTeamModal" />
  <PanelModalViewInvitations v-model="invitationsModal" />

  <PanelKing />
  <div class="absolute bottom-10 w-full flex flex-col gap-5 z-1">
    <ElevatedButton message="Utwórz zespół" class="w-70 h-15" @click="createTeamModal = true" />
    <ElevatedButton message="Zaproszenia" class="w-70 h-15" @click="invitationsModal = true" />
  </div>
</template>
