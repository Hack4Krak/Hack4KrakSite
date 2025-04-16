<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z
  .object({
    username: z.string({ error: 'Nazwa użytkownika jest wymagana' }).min(3, 'Nazwa użytkownika musi mieć min 3 znaki'),
  })

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  username: undefined,
})

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

const { $auth } = useNuxtApp()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  const response = await $auth('/account/update', {
    method: 'PATCH',
    body: {
      username: event.data.username,
    },
  })

  if ((response as any).error) {
    return
  }

  toast.add({ title: 'Sukces', description: 'Pomyślnie zaktualizowano ustawienia konta', color: 'success' })

  open.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Ustawienia konta" description="Zmień ustawienia konta" :ui="{ footer: 'justify-end' }">
    <template #body>
      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Nazwa użytkownika" name="username">
          <UInput v-model="state.username" class="w-full" />
        </UFormField>
      </UForm>
    </template>

    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Potwierdź" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
