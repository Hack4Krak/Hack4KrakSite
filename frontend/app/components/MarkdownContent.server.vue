<script setup lang="ts">
const props = defineProps<{
  text: string
  class?: string | undefined
  lightTheme?: boolean | undefined
}>()

const containerClasses = useState('markdown-container-classes', () => {
  return `prose ${props.lightTheme ? '' : 'prose-invert'}`
})
</script>

<template>
  <div :class="containerClasses">
    <MDC :value="text" :class="props.class" />
  </div>
</template>
