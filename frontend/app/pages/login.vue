<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Logowanie',
  description: 'Zaloguj się do swojego konta, aby móc brać udział w wydarzeniu!',
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
