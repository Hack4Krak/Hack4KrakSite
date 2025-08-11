<script setup lang="ts">
const props = withDefaults(defineProps<{
  text: string
  prose?: 'light' | 'dark' | 'none'
  inline?: boolean
}>(), {
  prose: 'dark',
  inline: false,
})

const proseClass = computed(() => {
  switch (props.prose) {
    case 'light': return 'prose'
    case 'dark': return 'prose prose-invert'
    default: return ''
  }
})

const inlineClass = computed(() => {
  return props.inline ? 'inline [&>p]:inline' : ''
})
</script>

<template>
  <span :class="proseClass">
    <MDC :value="text" :class="inlineClass" />
  </span>
</template>
