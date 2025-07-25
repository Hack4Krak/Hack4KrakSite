<script setup lang="ts">
const schema = z.object({
  username: zUsername().meta({ title: 'Nazwa użytkownika' }),
})

const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

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
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień ustawienia konta" :ui="{ footer: 'justify-end' }">
    <template #body>
      <AutoForm ref="form" :schema="schema" @submit="onSubmit" />
    </template>

    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Potwierdź" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
