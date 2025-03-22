<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

const { data: team } = await useAuth('/teams/membership/my_team')

const { data: invitedUsers } = await useAuth('/teams/management/invited_users', {
  onResponseError: undefined,
})

if (!team.value) {
  await navigateTo('/panel/')
}

const { error } = await useAuth('/teams/management/', {
  onResponseError: undefined,
})

const is_user_leader = error.value === undefined

const inviteUserModal = ref(false)
const kickedUser = ref('')
const revokedInvitation = ref('')

const { open: openDeleteTeamModal } = useConfirmModal({
  url: '/teams/management/delete',
  modalTitle: 'Usuwanie drużyny',
  modalDescription: 'Czy na pewno chcesz usunąć drużynę? Ta operacja jest nieodwracalna.',
  toastSuccessMessage: 'Pomyślnie usunięto drużynę',
  requestBody: undefined,
  redirectTo: '/panel/',
})
const { open: openLeaveTeamModal } = useConfirmModal({
  url: '/teams/membership/leave_team',
  modalTitle: 'Opuść drużynę',
  modalDescription: 'Czy na pewno chcesz opuścić drużynę? Ta operacja jest nieodwracalna.',
  toastSuccessMessage: 'Pomyślnie opuściłeś drużynę',
  requestBody: undefined,
  redirectTo: '/panel/',
})
const { open: openKickUserModal } = useConfirmModal({
  url: '/teams/management/kick_user',
  modalTitle: 'Wyrzucenie użytkownika',
  modalDescription: 'Czy na pewno chcesz opuścić drużynę? Ta operacja jest nieodwracalna.',
  toastSuccessMessage: 'Pomyślnie wyrzucono użytkownika',
  requestBody: { username: kickedUser },
  redirectTo: '/panel/team',
})
const { open: openRevokeInvitationModal } = useConfirmModal({
  url: `/teams/management/revoke_invitation/${revokedInvitation.value}` as any,
  modalTitle: 'Cofnięcie zaproszenia',
  modalDescription: 'Czy na pewno chcesz cofnąć zaproszenie?',
  toastSuccessMessage: 'Pomyślnie cofnięto zaproszenie',
  requestBody: undefined,
  redirectTo: '/panel/team',
})

type Members = ApiResponse<'my_team'>['members']

const members = computed<Members | null>(() => {
  const teamMembers = (team?.value?.members as Members) || []
  return [...teamMembers, ...Array.from({ length: 5 }).fill(null) as Members].slice(0, 5)
})
</script>

<template>
  <div>
    <PanelModalInviteUser v-model="inviteUserModal" />

    <NuxtLink to="/panel/" class="flex items-center gap-3 font-900 pt-5 pl-5">
      <Icon name="mdi:arrow-left" class="text-2xl" />
      Powrót
    </NuxtLink>
    <div class="flex flex-col md:flex-row md:mx-20 mx-10 gap-20 mt-10">
      <div class="border-2 border-neutral-600 p-5 min-w-70 rounded-2xl flex flex-col h-40">
        <h1 class="flex-grow text-3xl font-bold">
          {{ team?.team_name }}
        </h1>
        <UButton v-if="is_user_leader" class="w-full" @click="openDeleteTeamModal">
          <p class="text-center w-full">
            Usuń drużynę
          </p>
        </UButton>
        <UButton v-else class="w-full" @click="openLeaveTeamModal">
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
              <UButton v-if="is_user_leader" @click="openKickUserModal(); kickedUser = user.name">
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
        <div v-if="(invitedUsers ?? []).length > 0" class="border-2 border-neutral-600 flex-grow rounded-2xl">
          <div
            v-for="(user) in invitedUsers" :key="user"
            class="border-b-1 border-neutral-600 last-of-type:border-0 p-5"
          >
            <div class="flex justify-between items-center">
              {{ user }}
              <UButton v-if="is_user_leader" @click="openRevokeInvitationModal(); revokedInvitation = user">
                Cofnij zaproszenie
              </UButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
