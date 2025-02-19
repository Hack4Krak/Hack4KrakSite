<script setup lang="ts">
const props = defineProps<{
  url: string
  modalTitle: string
  modalDescription: string
  toastSuccessMessage: string
  requestBody: object | undefined
}>()

const toast = useToast()
const open = defineModel<boolean>()

async function onSubmit() {
  let request = {
    key: 'teams-create',
    method: 'DELETE',
  }
  console.error(props.requestBody)
  if (props.requestBody !== undefined) {
    request = Object.assign(request, { body: props.requestBody })
    console.error(request)
  }
  const { error } = await useAuth(props.url, request)

  if (error.value) {
    const response = error.value as any
    toast.add({ title: 'Błąd', description: response.message, color: 'error' })
  } else {
    toast.add({ title: 'Sukces', description: props.toastSuccessMessage, color: 'success' })
    navigateTo('/panel')
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
