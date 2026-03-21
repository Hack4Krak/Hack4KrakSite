<script setup lang="ts">
const schema = z.object({
  username: zUsername(),
})

const open = defineModel<boolean>()

async function onSubmit(data: zInfer<typeof schema>) {
  await useNuxtApp().$auth('/teams/management/invite_user', {
    method: 'POST',
    body: data,
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie zaproszono użytkownika', color: 'success' })
  open.value = false
  await refreshNuxtData()
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Zaproś użytkownika"
    :schema="schema"
    @submit="onSubmit"
  />
</template>
