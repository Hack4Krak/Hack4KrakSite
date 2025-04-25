<script setup>
definePageMeta({
  middleware: 'guest',
  layout: 'centered',
})
</script>

<template>
  <AuthForm :is-login="false" />
</template>
