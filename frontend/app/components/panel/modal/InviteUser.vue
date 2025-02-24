<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z.object({
  name: z.string({ required_error: 'Nazwa użytkownika' }).min(3, 'Nazwa uzytkownika musi mieć min 3 znaki'),
})

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  name: undefined,
})

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const { error } = await useAuth('/teams/management/invite_user', {
    key: 'teams-invite',
    method: 'POST',
    body: {
      username: event.data.name,
    },
  })

  if (error.value?.data === undefined) {
    toast.add({ title: 'Sukces', description: 'Pomyślnie zaproszono użytkownika', color: 'success' })
    open.value = false
    navigateTo('/panel/team', { external: true })
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Zaproś użytkownika" :ui="{ footer: 'justify-end' }">
    <template #body>
      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Username" name="name">
          <UInput v-model="state.name" class="w-full" />
        </UFormField>
      </UForm>
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Stwórz" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
