<script setup lang="ts">
defineProps<{ as?: string }>()
</script>

<template>
  <component :is="as ?? 'p'" class="text-xs font-bold uppercase tracking-widest text-primary">
    <slot />
  </component>
</template>
