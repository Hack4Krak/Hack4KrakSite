<script setup lang="ts">
const props = defineProps<{
  text: string
  class?: string
  prose?: 'light' | 'dark'
}>()

const proseClass = computed(() => {
  switch (props.prose) {
    case 'light': return 'prose'
    case 'dark': return 'prose prose-invert'
    default: return ''
  }
})
</script>

<template>
  <div :class="proseClass">
    <MDC :value="text" :class="props.class" />
  </div>
</template>
