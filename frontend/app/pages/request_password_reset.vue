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
  toast.add({ title: 'Sukces', description: 'Jeśli podany adres email jest powiązany z kontem, wkrótce otrzymasz link do zresetowania hasła', color: 'success' })

  await useNuxtApp().$api('/auth/request_reset_password', {
    method: 'POST',
    credentials: 'include',
    body: data,
  })
}
</script>

<template>
  <div>
    <h1 class="text-2xl font-medium">
      Zresetuj hasło
    </h1>

    <div class="flex-col gap-3 text-sm">
      <p>
        Bardzo nam przykro, że straciłeś dostęp do swojego konta :c
      </p>
      <p>
        Podaj swój adres email, a jeśli istnieje powiązane z nim konto, wyślemy link do zresetowania hasła.
      </p>
    </div>

    <AutoForm :schema="schema" @submit="onSubmit" />
  </div>
</template>
