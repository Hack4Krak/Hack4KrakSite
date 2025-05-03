<script setup lang="ts">
import file from '~/content/docs/School_Edition_Guidelines.md?raw'
</script>

<template>
  <div class="w-full mt-5 md:pl-[25svw] pl-[15svw]">
    <MarkdownContent :text="file" class="md:w-[50svw] w-[70svw]" prose="dark" />
  </div>
</template>
