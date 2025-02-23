<script setup lang="ts">
const { data: team } = await useAuth('/teams/membership/my_team', {
  key: 'teams-membership-my-team',
  method: 'GET',
})

let { data: invited_users } = await useAuth('/teams/management/invited_users', {
  key: 'teams-management-invited-users',
  method: 'GET',
})

invited_users = invited_users.value || []

if (!team.value) {
  await navigateTo('/panel/')
}

const { _, error } = await useAuth('/teams/management/', {
  key: 'is-user-leader',
  method: 'GET',
})

const is_user_leader = error.value === undefined

const inviteUserModal = ref(false)
const leaveTeamModal = ref(false)
const deleteTeamModal = ref(false)
const kickUserModal = ref(false)
const kickedUser = ref('')
const revokeInvitationModal = ref(false)
const revokedInvitation = ref('')

const members = computed(() => {
  const teamMembers = team?.value?.members || []
  return [...teamMembers, ...Array.from({ length: 5 }).fill(null)].slice(0, 5)
})
</script>

<template>
  <div>
    <PanelModalInviteUser v-model="inviteUserModal" />
    <PanelModalConfirmDeleteModal
      v-model="deleteTeamModal"
      url="/teams/management/delete"
      modal-title="Usuwanie drużyny"
      modal-description="Czy na pewno chcesz usunąć drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie usunięto drużynę"
      :request-body="undefined"
      redirect-to="/panel/"
    />
    <PanelModalConfirmDeleteModal
      v-model="leaveTeamModal"
      url="/teams/membership/leave_team"
      modal-title="Opuść drużynę"
      modal-description="Czy na pewno chcesz opuścić drużynę? Ta operacja jest nieodwracalna."
      toast-success-message="Pomyślnie opuściłeś drużynę"
      :request-body="undefined"
      redirect-to="/panel/"
    />
    <PanelModalConfirmDeleteModal
      v-model="kickUserModal"
      url="/teams/management/kick_user"
      modal-title="Wyrzucenie użytkownika"
      modal-description="Czy na pewno chcesz wyrzucić użytkownika z drużyny?"
      toast-success-message="Pomyślnie wyrzucono użytkownika"
      :request-body="{ username: kickedUser }"
      redirect-to="/panel/team"
    />
    <PanelModalConfirmDeleteModal
      v-model="revokeInvitationModal"
      :url="`/teams/management/revoke_invitation/${revokedInvitation}`"
      modal-title="Cofnięcie zaproszenia"
      modal-description="Czy na pewno chcesz cofnąć zaproszenie?"
      toast-success-message="Pomyślnie cofnięto zaproszenie"
      :request-body="undefined"
      redirect-to="/panel/team"
    />

    <NuxtLink to="/panel/" class="flex items-center gap-3 font-900 pt-5 pl-5">
      <Icon name="mdi:arrow-left" class="text-2xl" />
      Powrót
    </NuxtLink>
    <div class="flex flex-col md:flex-row md:mx-20 mx-10 gap-20 mt-10">
      <div class="border-2 border-neutral-600 p-5 min-w-70 rounded-2xl flex flex-col h-40">
        <h1 class="flex-grow text-3xl font-bold">
          {{ team?.team_name }}
        </h1>
        <UButton v-if="is_user_leader" class="w-full" @click="deleteTeamModal = true">
          <p class="text-center w-full">
            Usuń drużynę
          </p>
        </UButton>
        <UButton v-else class="w-full" @click="leaveTeamModal = true">
          <p class="text-center w-full">
            Opuść zespół
          </p>
        </UButton>
      </div>
      <div class="flex flex-col w-full gap-5">
        <div class="border-2 border-neutral-600 flex-grow rounded-2xl">
          <div
            v-for="(user, index) in members" :key="index"
            class="border-b-1 border-neutral-600 last-of-type:border-0 p-5"
          >
            <div v-if="user" class="flex justify-between items-center">
              <div>
                <UIcon v-if="user.is_leader" name="i-material-symbols-crown" class="text-yellow-400" />
                {{ user.name }}
              </div>
              <UButton v-if="is_user_leader" @click="kickUserModal = true; kickedUser = user.name">
                Wyrzuć
              </UButton>
            </div>
            <div v-else class="text-gray-400">
              <div v-if="is_user_leader" class="flex gap-5 items-center cursor-pointer" @click="inviteUserModal = true">
                <Icon name="mdi:account-plus" class="text-2xl" />
                Dodaj do zespołu
              </div>
            </div>
          </div>
        </div>
        <div v-if="invited_users.length > 0" class="border-2 border-neutral-600 flex-grow rounded-2xl">
          <div
            v-for="(user) in invited_users" :key="user"
            class="border-b-1 border-neutral-600 last-of-type:border-0 p-5"
          >
            <div class="flex justify-between items-center">
              {{ user }}
              <UButton v-if="is_user_leader" @click="revokeInvitationModal = true; revokedInvitation = user">
                Cofnij zaproszenie
              </UButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
