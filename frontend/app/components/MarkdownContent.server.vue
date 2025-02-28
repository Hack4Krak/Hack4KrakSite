<script setup lang="ts">
const props = defineProps<{
  text: string
  class?: string | undefined
}>()
</script>

<template>
  <div class="prose prose-invert">
    <MDC :value="props.text" :class="props.class" />
  </div>
</template>
