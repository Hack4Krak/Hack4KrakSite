<script setup lang="ts">
const props = defineProps<{
  text: string
  width: string | undefined
}>()
</script>

<template>
  <div :class="`prose prose-invert ${props.width}`">
    <MDC :value="props.text" :class="props.width" />
  </div>
</template>
