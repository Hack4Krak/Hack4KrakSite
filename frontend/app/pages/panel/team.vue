<script setup lang="ts">
const { data: team, refresh } = await useAuth('/teams/membership/my_team', {
  key: 'teams-membership-my-team',
  method: 'GET',
})

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

const members = computed(() => {
  const teamMembers = team?.value?.members || []
  return [...teamMembers, ...Array.from({ length: 5 }).fill(null)].slice(0, 5)
})

async function kick(username: string) {
  const { error } = await useAuth('/teams/management/kick_user', {
    key: 'teams-management-kick-user',
    method: 'DELETE',
    body: {
      username,
    },
  })

  const toast = useToast()

  if (error.value?.data) {
    const response = error.value.data as any
    toast.add({ title: 'Błąd', description: response.message, color: 'error' })
  } else {
    toast.add({ title: 'Sukces', description: 'Pomyślnie wyrzucono użytkownika', color: 'success' })
    await refresh()
  }
}
</script>

<template>
  <div>
    <PanelModalInviteUser v-model="inviteUserModal" />
    <PanelModalLeaveTeam v-model="leaveTeamModal" />
    <PanelModalDeleteTeam v-model="deleteTeamModal" />

    <NuxtLink to="/panel/" class="flex items-center gap-3 font-900 pt-5 pl-5">
      <Icon name="mdi:arrow-left" class="text-2xl" />
      Powrót
    </NuxtLink>
    <div class="flex mx-20 gap-20 mt-10">
      <div class="border-2 border-neutral-600 p-5 min-w-70 rounded-2xl flex flex-col">
        <h1 class="flex-grow text-3xl font-bold">
          {{ team?.team_name }}
        </h1>
        <UButton v-if="is_user_leader" class="w-full" @click="deleteTeamModal = true">
          Usuń zespół
        </UButton>
        <UButton v-else class="w-full" @click="leaveTeamModal = true">
          Opuść zespół
        </UButton>
      </div>
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
            <UButton v-if="is_user_leader" @click="kick(user.name)">
              Wyrzuć
            </UButton>
          </div>
          <div v-else class="text-gray-500 flex justify-between items-center">
            miejsce na uczestnika
            <UButton v-if="is_user_leader" @click="inviteUserModal = true">
              Dodaj do drużyny
            </UButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
