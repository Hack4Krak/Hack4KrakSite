<script setup lang="ts">
const schema = z.object({
  username: zUsername(),
})

const open = defineModel<boolean>()

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/account/update', {
    method: 'PATCH',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaktualizowano ustawienia konta', color: 'success' })
  open.value = false
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Ustawienia konta"
    description="Zmień ustawienia konta"
    :schema="schema"
    @submit="onSubmit"
  />
</template>
