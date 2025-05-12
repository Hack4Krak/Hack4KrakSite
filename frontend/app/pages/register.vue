<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})

useSeoMeta({
  title: 'Rejestracja',
})
</script>

<template>
  <AuthForm :is-login="false" />
</template>
