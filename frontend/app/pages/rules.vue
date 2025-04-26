<script setup lang="ts">
// Fallback
definePageMeta({
  middleware: () => {
    return navigateTo('/docs/rules')
  },
})
</script>

<template>
  <!-- Content will be rendered by the dynamic route -->
</template>
