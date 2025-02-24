<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z.object({
  name: z.string({ required_error: 'Nazwa druzyny jest wymagana' }).min(5, 'Nazwa drużyny musi mieć min 5 znaków'),
})

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  name: undefined,
})

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const { error } = await useAuth('/teams/create', {
    key: 'teams-create',
    method: 'POST',
    body: {
      team_name: event.data.name,
    },
  })

  if (error.value?.data === undefined) {
    toast.add({ title: 'Sukces', description: 'Pomyślnie stworzono team', color: 'success' })
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Stwórz team" description="Zbierz brygade swoich poteżnych team-matów do walki!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Nazwa" name="name">
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
