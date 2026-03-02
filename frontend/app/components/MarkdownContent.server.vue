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

const parse = useMarkdownParser()
const ast = await parse(props.text)
</script>

<template>
  <div :class="proseClass">
    <MDCRenderer v-if="ast?.body" :body="ast.body" :data="ast.data" :class="inlineClass" />
  </div>
</template>
