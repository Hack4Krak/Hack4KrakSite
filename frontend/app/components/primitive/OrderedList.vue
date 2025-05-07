<script setup lang="ts">
defineProps<{
  items: string[]
}>()
</script>

<template>
  <ol class="lg:max-w-1/2 mx-auto mt-6 mb-2 flex flex-col text-left prose dark:prose-invert">
    <li v-for="(sentence, index) in items" :key="index">
      <span class="font-bold">{{ index + 1 }}. </span>
      <MarkdownContent :text="sentence" :inline="true" />
    </li>
  </ol>
</template>
