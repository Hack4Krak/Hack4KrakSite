<script setup lang="ts">
const schema = z.object({
  username: zUsername().meta({ title: 'Nazwa użytkownika' }),
})

const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/teams/management/invite_user', {
    method: 'POST',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaproszono użytkownika', color: 'success' })
  open.value = false
  navigateTo('/panel/team', { external: true })
}
</script>

<template>
  <UModal v-model:open="open" title="Zaproś użytkownika" :ui="{ footer: 'justify-end' }">
    <template #body>
      <AutoForm ref="form" :schema="schema" @submit="onSubmit" />
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Stwórz" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
