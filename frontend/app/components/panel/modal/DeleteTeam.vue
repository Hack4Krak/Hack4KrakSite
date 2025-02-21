<script setup lang="ts">
const toast = useToast()
const open = defineModel<boolean>()

async function onSubmit() {
  const { error } = await useAuth('/teams/management/delete', {
    key: 'teams-create',
    method: 'DELETE',
  })

  if (error.value) {
    const response = error.value as any
    toast.add({ title: 'Błąd', description: response.message, color: 'error' })
  } else {
    toast.add({ title: 'Sukces', description: 'Pomyślnie usunięto drużynę', color: 'success' })
    navigateTo('/panel')
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Usuń drużynę" description="Czy na pewno chcesz usunąć drużynę?" :ui="{ footer: 'justify-end' }">
    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Usuń" @click="onSubmit()" />
    </template>
  </UModal>
</template>
