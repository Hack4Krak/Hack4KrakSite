<script setup lang="ts">
import type { paths } from '#open-fetch-schemas/auth'

const props = defineProps<{
  url: keyof paths
  modalTitle: string
  modalDescription: string
  toastSuccessMessage: string
  requestBody: object | undefined
  redirectTo: string
}>()

const toast = useToast()
const open = defineModel<boolean>()
const schema = z.object({})

const { $auth } = useNuxtApp()

async function onSubmit() {
  const response = await $auth(props.url, {
    method: 'DELETE',
    ...(props.requestBody && { body: props.requestBody }),
  } as any).catch()

  if ((response as any).error) {
    return
  }

  toast.add({ title: 'Sukces', description: props.toastSuccessMessage, color: 'success' })
  open.value = false

  await refreshNuxtData()
  await navigateTo(props.redirectTo)
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    :schema="schema"
    :title="modalTitle"
    :description="modalDescription"
    submit-label="Potwierdź"
    close-label="Anuluj"
    @submit="onSubmit"
  />
</template>
