<script setup lang="ts">
import type { paths } from '#open-fetch-schemas/auth'

const props = defineProps<{
  url: keyof paths
  modalTitle: string
  modalDescription: string
  toastSuccessMessage: string
  requestBody: object | undefined
  redirectTo: string
  confirmLabel?: string
}>()
const emit = defineEmits<{
  success: []
}>()

const toast = useToast()
const open = defineModel<boolean>()

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
  emit('success')

  await refreshNuxtData()
  await navigateTo(props.redirectTo)
}
</script>

<template>
  <UModal v-model:open="open" :title="modalTitle" :description="modalDescription" :ui="{ footer: 'justify-end' }">
    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton :label="confirmLabel ?? 'Usuń'" @click="onSubmit()" />
    </template>
  </UModal>
</template>
