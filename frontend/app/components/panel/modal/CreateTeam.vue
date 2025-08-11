<script setup lang="ts">
const schema = z.object({
  team_name: zTeamName().meta({ title: 'Nazwa drużyny' }),
})

const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/teams/create', {
    method: 'POST',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie stworzono team', color: 'success' })
  open.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Stwórz team" description="Zbierz brygadę swoich potężnych team-matów do walki!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <AutoForm ref="form" :schema="schema" @submit="onSubmit" />
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Stwórz" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
