<script setup>
definePageMeta({
  middleware: 'guest',
})
const route = useRoute()
if (route.query.error) {
  useToast().clear()
  useToast().add({ title: 'Nie udało się zalogować', description: route.query.error, color: 'error' })

  const query = Object.assign({}, route.query)
  delete query.error
  useRouter().replace({ query })
}
</script>

<template>
  <Auth :is-login="true" />
</template>
