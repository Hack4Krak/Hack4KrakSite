<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

const route = useRoute()

if (route.query.error) {
  await callOnce(() => useToast().add({ title: 'Nie udało się zalogować', description: route.query.error, color: 'error' }))
  const query = Object.assign({}, route.query)
  delete query.error
  useRouter().replace({ query })
}
</script>

<template>
  <AuthForm :is-login="true" class="flex-grow" />
</template>
