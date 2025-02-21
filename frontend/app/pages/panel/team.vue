<script setup lang="ts">
const { data: team, refresh } = await useAuth('/teams/membership/my_team', {
  key: 'teams-membership-my-team',
  method: 'GET',
})

const inviteUserModal = ref(false)

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

    <NuxtLink to="/panel/">
      Wróć
    </NuxtLink>
    <div class="flex mx-20 gap-20">
      <div class="border-2 border-neutral-600 p-5 min-w-70 rounded-2xl flex flex-col">
        <h1 class="flex-grow text-3xl font-bold">
          {{ team?.team_name }}
        </h1>
        <UButton class="w-full">
          Usuń
        </UButton>
      </div>
      <div class="border-2 border-neutral-600 flex-grow rounded-2xl">
        <div
          v-for="(user, index) in members" :key="index"
          class="border-b-1 border-neutral-600 last-of-type:border-0 p-5"
        >
          <div v-if="user">
            <UIcon v-if="user.is_leader" name="i-material-symbols-crown" class="text-yellow-400" />
            {{ user.name }}
            <UButton @click="kick(user.name)">
              Wyrzuć
            </UButton>
          </div>
          <div v-else>
            <UButton @click="inviteUserModal = true">
              Dodaj
            </UButton>
            Dodaj do zespołu
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
