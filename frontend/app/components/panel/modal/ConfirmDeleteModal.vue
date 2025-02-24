<script setup lang="ts">
const props = defineProps<{
  url: string
  modalTitle: string
  modalDescription: string
  toastSuccessMessage: string
  requestBody: object | undefined
  redirectTo: string
}>()

const { $auth } = useNuxtApp()

const toast = useToast()
const open = defineModel<boolean>()

async function onSubmit() {
  let request = {
    key: `${props.url}-modal`,
    method: 'DELETE',
  }
  console.error(props.requestBody)
  if (props.requestBody !== undefined) {
    request = Object.assign(request, { body: props.requestBody })
  }
  const data: any = await $auth(props.url, request)

  if (data.error != null) {
    toast.add({ title: 'Błąd', description: data.message, color: 'error' })
  } else {
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
