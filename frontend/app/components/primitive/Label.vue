<script setup lang="ts">
const { text } = defineProps<{
  text: string
}>()
</script>

<template>
  <div class="px-4 py-2 ring-2 ring-accent-primary">
    {{ text }}
  </div>
</template>
