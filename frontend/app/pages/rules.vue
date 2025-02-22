<script setup lang="ts">
import file from 'assets/md/Regulamin_Hack4Krak_LO31.md?raw'

const regulations = file
</script>

<template>
  <div class="flex w-full items-center justify-center mt-5">
    <MarkdownContent :text="regulations" />
  </div>
</template>
