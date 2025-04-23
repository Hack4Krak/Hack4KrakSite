<script setup lang="ts">
const createTeamModal = ref(false)
const invitationsModal = ref(false)
</script>

<template>
  <PanelModalCreateTeam v-model="createTeamModal" />
  <PanelModalViewInvitations v-model="invitationsModal" />

  <PanelKing />
  <div class="ml-12 absolute bottom-10 w-full flex flex-col gap-5 z-1">
    <ElevatedButton class="w-70" @click="createTeamModal = true">
      Utwórz zespół
    </ElevatedButton>
    <ElevatedButton class="w-70" @click="invitationsModal = true">
      Zaproszenia
    </ElevatedButton>
  </div>
</template>
