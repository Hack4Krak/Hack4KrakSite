<script setup lang="ts">
const schema = z.object({
  username: zUsername().meta({ title: 'Nazwa użytkownika' }),
})

const open = defineModel<boolean>()

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/account/update', {
    method: 'PATCH',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaktualizowano ustawienia konta', color: 'success' })
  open.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień ustawienia konta">
    <template #body>
      <AutoForm :schema="schema" @submit="onSubmit" />
    </template>
  </UModal>
</template>
