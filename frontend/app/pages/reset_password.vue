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
  new_password: z.string({ error: 'Hasło jest wymagane' }).min(8, 'Hasło musi mieć minimum 8 znaków'),
})

const route = useRoute()

const code = route.query.code
if (code === undefined) {
  const toast = useToast()
  toast.add({ title: 'Błąd', description: 'Niepoprawny link do resetowania hasła', color: 'error' })
  navigateTo('/request_password_reset')
}

const loading = ref(false)
const toast = useToast()
const state = reactive<Partial<Schema>>({
  new_password: undefined,
})

const isButtonEnabled = computed(() => {
  return schema.safeParse(state).success
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  loading.value = true

  try {
    await useNuxtApp().$api('/auth/reset_password', {
      method: 'PATCH',
      credentials: 'include',
      body: {
        code: code as string,
        new_password: event.data.new_password,
      },
    })

    toast.add({ title: 'Sukces', description: 'Pomyślnie ustawiono nowe hasło. Możesz się teraz zalogować', color: 'success' })
    await navigateTo('/login')
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
  <div class="sm:w-90 w-60">
    <h1 class="text-2xl font-medium">
      Zresetuj hasło
    </h1>

    <UForm :schema="schema" :state="state" class="space-y-4 text-center" @submit="onSubmit">
      <UFormField label="Nowe hasło" name="new_password">
        <TransparentInput v-model="state.new_password" />
      </UFormField>

      <div class="space-y-2">
        <UButton type="submit" class="w-full text-center inline rounded-3xl py-2 bg-neutral-300" :disabled="loading" :class="isButtonEnabled ? 'bg-primary' : ''">
          Potwierdż
        </UButton>
      </div>
    </UForm>
  </div>
</template>
