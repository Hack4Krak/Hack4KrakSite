<script setup lang="ts">
const props = defineProps<{
  icon: string
  description: string
}>()
</script>

<template>
  <UButton variant="ghost">
    <Icon :name="props.icon" class="text-2xl" />
    {{ props.description }}
  </UButton>
</template>
