<script setup lang="ts">
import type { FormSubmitEvent } from '#ui/types'
import { FetchError } from 'ofetch'
import * as z from 'zod'

definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

type Schema = z.output<typeof schema>

const schema = z.object({
  email: z.email({ error: 'Adres e-mail jest wymagany' }),
})

const loading = ref(false)
const toast = useToast()
const state = reactive<Partial<Schema>>({
  email: undefined,
})

const isButtonEnabled = computed(() => {
  return schema.safeParse(state).success
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  loading.value = true

  try {
    toast.add({ title: 'Oczekiwanie', description: 'Wysyłanie emaila…', color: 'info' })

    await useNuxtApp().$api('/auth/request_reset_password', {
      method: 'POST',
      credentials: 'include',
      body: event.data,
    })

    toast.add({ title: 'Sukces', description: 'Pomyślnie wysłano link do resetowania hasła', color: 'success' })
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }
  } finally {
    loading.value = false
  }
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

    <UForm :schema="schema" :state="state" class="space-y-4 text-center" @submit="onSubmit">
      <UFormField label="Email" name="email">
        <TransparentInput v-model="state.email" type="email" />
      </UFormField>

      <div class="space-y-2">
        <UButton type="submit" class="w-full text-center inline rounded-3xl py-2 bg-neutral-300" :disabled="loading" :class="isButtonEnabled ? 'bg-(--ui-primary)' : ''">
          Wyślij
        </UButton>
      </div>
    </UForm>
  </div>
</template>
