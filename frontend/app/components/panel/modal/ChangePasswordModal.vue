<script setup lang="ts">
const schema = z
  .object({
    new_password: zPassword(),
    confirm_new_password: zPassword('Potwierdzenie hasła jest wymagane'),
    old_password: zPassword('Stare hasło jest wymagane'),
  })
  .check(z.refine(data => data.new_password === data.confirm_new_password, {
    message: 'Hasła nie są takie same',
    path: ['confirm_new_password'],
  }))

const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/account/update/password', {
    method: 'PATCH',
    body: {
      old_password: data.old_password,
      new_password: data.new_password,
    },
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaktualizowano ustawienia konta', color: 'success' })
  open.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień hasło" :ui="{ footer: 'justify-end' }">
    <template #body>
      <AutoForm ref="form" :schema="schema" @submit="onSubmit" />
      <NuxtLink class="link" to="/request_password_reset">
        Zresetuj hasło
      </NuxtLink>
    </template>

    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Potwierdź" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
