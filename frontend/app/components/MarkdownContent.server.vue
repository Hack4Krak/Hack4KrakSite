<script setup lang="ts">
const props = defineProps<{
  text: string
  class?: string | undefined
  lightTheme?: boolean | undefined
}>()
</script>

<template>
  <div class="prose {{ props.lightTheme ? '' : 'prose-invert' }}">
    <MDC :value="text" :class="props.class" />
  </div>
</template>
