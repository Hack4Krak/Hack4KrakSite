<script setup lang="ts">
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Resetowanie hasła',
  description: 'Zresetuj hasło do swojego konta, aby móc brać udział w wydarzeniu!',
})

const toast = useToast()
const schema = z.object({
  email: z.email().meta({ title: 'Email' }),
})

async function onSubmit(data: zInfer<typeof schema>) {
  toast.add({ title: 'Oczekiwanie', description: 'Wysyłanie emaila…', color: 'info' })

  await useNuxtApp().$api('/auth/request_reset_password', {
    method: 'POST',
    credentials: 'include',
    body: data,
  })

  toast.add({ title: 'Sukces', description: 'Pomyślnie wysłano link do resetowania hasła', color: 'success' })
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
        Podaj swój adres, na który wyślemy wiadomość do potwierdzenia maila
      </p>
    </div>

    <AutoForm :schema="schema" @submit="onSubmit" />
  </div>
</template>
