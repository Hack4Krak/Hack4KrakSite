<script setup lang="ts">
const { data, refresh } = await useAuth('/teams/invitations/', {
  key: 'teams-invitations',
})

const open = defineModel<boolean>()

watch(open, (newValue) => {
  if (newValue) {
    refresh()
  }
})

async function accept(team_name: string) {
  const { error } = await useAuth('/teams/invitations/accept_invitation/{team_name}', {
    key: 'accept-invitation',
    method: 'POST',
    path: { team_name },
  })

  const toast = useToast()

  if (error.value?.data) {
    const response = error.value.data as any
    toast.add({ title: 'Błąd', description: response.message, color: 'error' })
  } else {
    toast.add({ title: 'Sukces', description: 'Pomyślnie zaakceptowano użytkownika', color: 'success' })
    await navigateTo('/panel/team')
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Zaproszenia" description="Zbierz brygade swoich poteżnych team-matów do walki!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <div v-if="data?.length === 0">
        Brak danych do wyświetlenia
      </div>
      <div v-for="team in data" :key="team" class="flex justify-between">
        {{ team }} <UButton @click="accept(team)">
          Zaakceptuj
        </UButton>
      </div>
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
    </template>
  </UModal>
</template>
