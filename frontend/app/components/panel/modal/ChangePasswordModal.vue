<script setup lang="ts">
const schema = z
  .object({
    old_password: zPassword('Stare hasło jest wymagane').meta({ title: 'Stare hasło' }),
    new_password: zPassword('Nowe hasło jest wymagane').meta({ title: 'Nowe hasło' }),
    confirm_new_password: zPassword('Potwierdzenie hasła jest wymagane').meta({ title: 'Powtórz nowe hasło' }),
  })
  .check(z.refine(data => data.new_password === data.confirm_new_password, {
    message: 'Hasła nie są takie same',
    path: ['confirm_new_password'],
  }))

const open = defineModel<boolean>()

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
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień hasło">
    <template #body>
      <AutoForm :schema="schema" @submit="onSubmit" />
      <NuxtLink class="link" to="/request_password_reset">
        Zresetuj hasło
      </NuxtLink>
    </template>
  </UModal>
</template>
