<script setup lang="ts">
const props = withDefaults(defineProps<{
  text: string
  prose?: boolean
  inline?: boolean
}>(), {
  prose: true,
  inline: false,
})

const proseClass = computed(() => props.prose ? 'max-w-[65ch]' : '')

const inlineClass = computed(() => {
  return props.inline ? 'inline [&>p]:inline' : ''
})
</script>

<template>
  <div :class="proseClass">
    <MDC :value="text" :class="inlineClass" />
  </div>
</template>
