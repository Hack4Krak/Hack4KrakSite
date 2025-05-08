<script setup lang="ts">
const createTeamModal = ref(false)
const joinExternalTeamModal = ref(false)
const invitationsModal = ref(false)

const { data: isExternalRegistration } = useApi('/event/registration', {
  transform: (data) => {
    return data.registration_mode === 'external'
  },
})
</script>

<template>
  <PanelModalCreateTeam v-model="createTeamModal" />
  <PanelModalViewInvitations v-model="invitationsModal" />
  <PanelModalJoinExternalTeam v-model="joinExternalTeamModal" />

  <PanelKing />
  <div class="ml-12 absolute bottom-10 w-full flex flex-col gap-5 z-1">
    <div v-if="isExternalRegistration">
      <ElevatedButton class="w-70" @click="joinExternalTeamModal = true">
        Dołącz do zespołu
      </ElevatedButton>
    </div>
    <div v-else>
      <ElevatedButton class="w-70" @click="createTeamModal = true">
        Utwórz zespół
      </ElevatedButton>
      <ElevatedButton class="w-70" @click="invitationsModal = true">
        Zaproszenia
      </ElevatedButton>
    </div>
  </div>
</template>
