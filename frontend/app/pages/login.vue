<script setup>
definePageMeta({
  middleware: 'guest',
})
</script>

<template>
  <Auth :is-login="true" />
</template>
