<script setup lang="ts">
definePageMeta({
  layout: 'centered',
})

useSeoMeta({
  title: 'Resetowanie hasła',
  description: 'Zresetuj hasło do swojego konta, aby móc brać udział w wydarzeniu!',
})

const toast = useToast()
const schema = z.object({
  email: z.email({ error: 'Niepoprawny adres e-mail' }).meta({ title: 'Adres e-mail' }),
})

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$api('/auth/request_reset_password', {
    method: 'POST',
    credentials: 'include',
    body: data,
  })

  toast.add({ title: 'Sukces', description: 'Jeśli podany adres email jest powiązany z kontem, wkrótce otrzymasz link do zresetowania hasła.', color: 'success' })
  await navigateTo('/login')
}
</script>

<template>
  <div class="space-y-4">
    <h1 class="text-2xl font-medium">
      Zresetuj hasło
    </h1>

    <p class="text-sm">
      Podaj swój adres email, a jeśli istnieje powiązane z nim konto, wyślemy link do zresetowania hasła.
    </p>

    <AutoForm :schema="schema" @submit="onSubmit" />

    <div class="w-full text-center">
      <span class="text-sm text-neutral-400">
        Pamiętasz hasło?
        <NuxtLink class="link" to="/login">
          Zaloguj się
        </NuxtLink>
      </span>
    </div>
  </div>
</template>
