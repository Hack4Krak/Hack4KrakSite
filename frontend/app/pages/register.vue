<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Rejestracja',
  description: 'Zarejestruj się, aby móc brać udział w wydarzeniu! Stwórz drużynę i walcz o nagrody!',
})
</script>

<template>
  <AuthForm :is-login="false" />
</template>
