<script setup lang="ts">
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Resetowanie hasła',
  description: 'Zresetuj hasło do swojego konta, aby móc brać udział w wydarzeniu!',
})

const schema = z.object({
  new_password: zPassword().meta({ title: 'Nowe hasło' }),
})

const code = useRoute().query.code
if (code === undefined) {
  useToast().add({ title: 'Błąd', description: 'Niepoprawny link do resetowania hasła', color: 'error' })
  navigateTo('/request_password_reset')
}

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$api('/auth/reset_password', {
    method: 'PATCH',
    credentials: 'include',
    body: {
      code: code as string,
      new_password: data.new_password,
    },
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie ustawiono nowe hasło. Możesz się teraz zalogować', color: 'success' })
  await navigateTo('/login')
}
</script>

<template>
  <div>
    <h1 class="text-2xl font-medium">
      Zresetuj hasło
    </h1>

    <AutoForm :schema="schema" @submit="onSubmit" />
  </div>
</template>
