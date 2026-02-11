<script setup lang="ts">
const schema = z.object({
  username: zUsername().meta({ title: 'Nazwa użytkownika' }),
})

const open = defineModel<boolean>()

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/teams/management/invite_user', {
    method: 'POST',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaproszono użytkownika', color: 'success' })
  open.value = false
  await refreshNuxtData()
}
</script>

<template>
  <UModal v-model:open="open" title="Zaproś użytkownika">
    <template #body>
      <AutoForm :schema="schema" @submit="onSubmit" />
    </template>
  </UModal>
</template>
