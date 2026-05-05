<script setup lang="ts">
const schema = z.object({
  team_name: zTeamName(),
})

const open = defineModel<boolean>()

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/teams/create', {
    method: 'POST',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie stworzono team', color: 'success' })
  await refreshNuxtData()
  open.value = false
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Stwórz team"
    description="Zbierz brygadę swoich potężnych team-matów do walki!"
    :schema="schema"
    @submit="onSubmit"
  />
</template>
