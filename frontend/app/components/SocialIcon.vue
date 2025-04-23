<script setup lang="ts">
defineProps<{
  icon: string
  link: string
}>()
</script>

<template>
  <NuxtLink :to="link" target="_blank">
    <UIcon :name="icon" class="size-8" />
  </NuxtLink>
</template>
