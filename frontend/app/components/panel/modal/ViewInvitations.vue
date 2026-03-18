<script setup lang="ts">
const { data, refresh } = await useAuth('/teams/invitations/', {
  key: 'teams-invitations',
})

const { $auth } = useNuxtApp()

const open = defineModel<boolean>()
const schema = z.object({})

watch(open, (newValue) => {
  if (newValue) {
    refresh()
  }
})

async function accept(team_name: string) {
  const response = await $auth('/teams/invitations/accept_invitation/{team_name}', {
    method: 'POST',
    path: { team_name },
  }).catch()

  if ((response as any).error) {
    return
  }

  const toast = useToast()
  toast.add({ title: 'Sukces', description: 'Pomyślnie zaakceptowano użytkownika', color: 'success' })
  await navigateTo('/panel/team')
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Zaproszenia"
    description="Zbierz brygade swoich poteżnych team-matów do walki!"
    :schema="schema"
    :config="{ submit: false }"
    @submit="open = false"
  >
    <template #after-fields>
      <div v-if="data?.length === 0">
        Brak danych do wyświetlenia
      </div>
      <div v-for="team in data" :key="team" class="flex justify-between">
        {{ team }} <UButton @click="accept(team)">
          Zaakceptuj
        </UButton>
      </div>
    </template>

    <template #footer="{ close }">
      <UButton label="Zamknij" color="neutral" variant="outline" @click="close" />
    </template>
  </AutoFormModal>
</template>
