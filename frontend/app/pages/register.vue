<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Rejestracja',
  description: 'Zarejestruj się, aby móc brać udział w wydarzeniu! Stwórz drużynę i walcz o nagrody!',
})

defineOgImage('DefaultOgImage', { bg: '/img/landing_background.webp' })
</script>

<template>
  <AuthForm :is-login="false" />
</template>
