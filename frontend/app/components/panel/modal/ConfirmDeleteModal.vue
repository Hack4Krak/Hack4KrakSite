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

async function onSubmit() {
  const { error } = await useAuth(props.url, {
    method: 'DELETE',
    ...(props.requestBody && { body: props.requestBody }),
  } as any)

  if (error.value === undefined) {
    toast.add({ title: 'Sukces', description: props.toastSuccessMessage, color: 'success' })
    open.value = false
    navigateTo(props.redirectTo, { external: true })
  }
}
</script>

<template>
  <UModal v-model:open="open" :title="props.modalTitle" :description="props.modalDescription" :ui="{ footer: 'justify-end' }">
    <template #footer>
      <UButton label="Anuluj" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Potwierdź" @click="onSubmit()" />
    </template>
  </UModal>
</template>
