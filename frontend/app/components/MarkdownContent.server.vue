<script setup lang="ts">
const props = defineProps<{
  text: string
}>()
</script>

<template>
  <div class="prose prose-invert">
    <MDC :value="props.text" />
  </div>
</template>
